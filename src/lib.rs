mod utils;

use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO: Use failure
// TODO: Use format for the error in wasm_decode
// TODO: Add adjustable 'punch'

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    LengthInvalid,
    LengthMismatch,
}

/* Decode for WASM target
 * It is similar to `decode`, but uses an option for the Error
 * I could not figure out a good way to use the Error in the regular decode,
 * and was getting a E0277 or not being able to convert automatically.

 * There are two current options, afaict:
 *  1) Result E be a JsValue with hardcoded (or formatted) strings.
 *  2) Transform the Result to Option, which on failure would be undefined in JS.

 * For convenience, I went with 2), until we can return Error.
 * This seems to be an open topic atm, so a separate decode seems ok for now :)
 * @see https://github.com/rustwasm/wasm-bindgen/issues/1017
*/
#[wasm_bindgen(js_name = "decode")]
pub fn wasm_decode(blur_hash: &str, width: usize, height: usize) -> Option<Vec<u8>> {
    match decode(blur_hash, width, height) {
        Ok(img) => Some(img),
        Err(_err) => None,
    }
}

pub fn decode(blur_hash: &str, width: usize, height: usize) -> Result<Vec<u8>, Error> {
    if blur_hash.len() < 6 {
        return Err(Error::LengthInvalid);
    }

    // 1. Number of components
    // For a BlurHash with nx components along the X axis and ny components
    // along the Y axis, this is equal to (nx - 1) + (ny - 1) * 9.
    let size_flag = decode_base83_string(blur_hash.get(0..1).unwrap());

    let num_y = (size_flag / 9) + 1;
    let num_x = (size_flag % 9) + 1;

    // Validate that the number of digits is what we expect:
    // 1 (size flag) + 1 (maximum value) + 4 (average colour) + (num_x - num_y - 1) components * 2 digits each
    let expected_digits = 4 + 2 * num_x * num_y;

    if blur_hash.len() != expected_digits {
        return Err(Error::LengthMismatch);
    }

    // 2. Maximum AC component value, 1 digit.
    // All AC components are scaled by this value.
    // It represents a floating-point value of (max + 1) / 166.
    let quantised_maximum_value = decode_base83_string(blur_hash.get(1..2).unwrap());
    let maximum_value = ((quantised_maximum_value + 1) as f64) / 166f64;

    let mut colours: Vec<[f64; 3]> = Vec::new();

    for i in 0..(num_x * num_y) {
        if i == 0 {
            // 3. Average colour, 4 digits.
            let value = decode_base83_string(blur_hash.get(2..6).unwrap());
            colours.push(decode_dc(value));
        } else {
            // 4. AC components, 2 digits each, nx * ny - 1 components in total.
            let value = decode_base83_string(blur_hash.get((4 + i * 2)..(4 + i * 2 + 2)).unwrap());
            colours.push(decode_ac(value, maximum_value * 1.0));
        }
    }

    // Now, construct the image
    // NOTE: We include an alpha channel of 255 as well, because it is more convenient,
    // for various representations (browser canvas, for example).
    // This could probably be configured
    let bytes_per_row = width * 4;

    let mut pixels = vec![0; bytes_per_row * height];

    for y in 0..height {
        for x in 0..width {
            let mut r = 0f64;
            let mut g = 0f64;
            let mut b = 0f64;

            for j in 0..num_y {
                for i in 0..num_x {
                    let basis = f64::cos(PI * (x as f64) * (i as f64) / (width as f64))
                        * f64::cos(PI * (y as f64) * (j as f64) / (height as f64));
                    let colour = colours[i + j * num_x];
                    r += colour[0] * basis;
                    g += colour[1] * basis;
                    b += colour[2] * basis;
                }
            }

            let int_r = linear_to_srgb(r);
            let int_g = linear_to_srgb(g);
            let int_b = linear_to_srgb(b);

            pixels[4 * x + 0 + y * bytes_per_row] = int_r;
            pixels[4 * x + 1 + y * bytes_per_row] = int_g;
            pixels[4 * x + 2 + y * bytes_per_row] = int_b;
            pixels[4 * x + 3 + y * bytes_per_row] = 255;
        }
    }

    Ok(pixels)
}

fn decode_dc(value: usize) -> [f64; 3] {
    let int_r = value >> 16;
    let int_g = (value >> 8) & 255;
    let int_b = value & 255;
    [
        srgb_to_linear(int_r),
        srgb_to_linear(int_g),
        srgb_to_linear(int_b),
    ]
}

fn decode_ac(value: usize, maximum_value: f64) -> [f64; 3] {
    let quant_r = f64::floor((value / (19 * 19)) as f64);
    let quant_g = f64::floor(((value / 19) as f64) % 19f64);
    let quant_b = (value as f64) % 19f64;

    let rgb = [
        sign_pow((quant_r - 9f64) / 9f64, 2f64) * maximum_value,
        sign_pow((quant_g - 9f64) / 9f64, 2f64) * maximum_value,
        sign_pow((quant_b - 9f64) / 9f64, 2f64) * maximum_value,
    ];
    rgb
}

fn sign_pow(value: f64, exp: f64) -> f64 {
    get_sign(value) * f64::powf(f64::abs(value), exp)
}

fn get_sign(n: f64) -> f64 {
    if n < 0f64 {
        -1f64
    } else {
        1f64
    }
}

fn linear_to_srgb(value: f64) -> u8 {
    let v = f64::max(0f64, f64::min(1f64, value));
    if v <= 0.0031308 {
        return (v * 12.92 * 255f64 + 0.5) as u8;
    } else {
        return ((1.055 * f64::powf(v, 1f64 / 2.4) - 0.055) * 255f64 + 0.5) as u8;
    }
}

fn srgb_to_linear(value: usize) -> f64 {
    let v = (value as f64) / 255f64;
    if v <= 0.04045 {
        return v / 12.92;
    } else {
        return ((v + 0.055) / 1.055).powf(2.4);
    }
}

// TODO: Consider using lazy_static to expand this, or even write long-hand
const ENCODE_CHARACTERS: &str =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~";

fn decode_base83_string(string: &str) -> usize {
    let mut value: usize = 0;

    for character in string.chars() {
        match ENCODE_CHARACTERS.find(character) {
            Some(digit) => value = value * 83 + digit,

            None => (),
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_size_flag() {
        assert_eq!(21, decode_base83_string("L"));
        assert_eq!(0, decode_base83_string("0"));
    }
    #[test]
    fn decodes_size_0_out_of_range() {
        let res = decode_base83_string("/");
        assert_eq!(
            0, res,
            "Did not expect to decode size for input out of range (expected 0), but got {}",
            res
        );
    }
}
