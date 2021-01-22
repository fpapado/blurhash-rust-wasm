mod utils;

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::f64::consts::PI;
use thiserror::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO: Add adjustable 'punch'
// TODO: Avoid panicing infrasturcture (checked division, .get, no unwrap)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("the length of the hash is invalid")]
    LengthInvalid,
    #[error("the specified number of components does not match the actual length")]
    LengthMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum EncodingError {
    #[error("cannot encode this number of components")]
    ComponentsNumberInvalid,
    #[error("the bytes per pixel does not match the pixel count")]
    BytesPerPixelMismatch,
}

// Decode

/// Decode for WASM target. If an error occurs, the function will throw a `JsError`.
#[wasm_bindgen(js_name = "decode")]
pub fn wasm_decode(blur_hash: &str, width: usize, height: usize) -> Result<Vec<u8>, JsValue> {
    decode(blur_hash, width, height).map_err(|err| js_sys::Error::new(&err.to_string()).into())
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

    let mut colours: Vec<[f64; 3]> = Vec::with_capacity(num_x * num_y);

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

            pixels[4 * x + y * bytes_per_row] = int_r as u8;
            pixels[4 * x + 1 + y * bytes_per_row] = int_g as u8;
            pixels[4 * x + 2 + y * bytes_per_row] = int_b as u8;
            pixels[4 * x + 3 + y * bytes_per_row] = 255 as u8;
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
    [
        sign_pow((quant_r - 9f64) / 9f64, 2f64) * maximum_value,
        sign_pow((quant_g - 9f64) / 9f64, 2f64) * maximum_value,
        sign_pow((quant_b - 9f64) / 9f64, 2f64) * maximum_value,
    ]
}

fn sign_pow(value: f64, exp: f64) -> f64 {
    value.abs().powf(exp).copysign(value)
}

fn linear_to_srgb(value: f64) -> usize {
    let v = f64::max(0f64, f64::min(1f64, value));
    if v <= 0.003_130_8 {
        (v * 12.92 * 255f64 + 0.5) as usize
    } else {
        ((1.055 * f64::powf(v, 1f64 / 2.4) - 0.055) * 255f64 + 0.5) as usize
    }
}

fn srgb_to_linear(value: usize) -> f64 {
    let v = (value as f64) / 255f64;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

// Encode

// TODO: Think about argument order here...
// What is more common in Rust? Data or config first?
pub fn encode(
    pixels: Vec<u8>,
    cx: usize,
    cy: usize,
    width: usize,
    height: usize,
) -> Result<String, EncodingError> {
    // Should we assume RGBA for round-trips? Or does it not matter?
    let bytes_per_row = width * 4;
    let bytes_per_pixel = 4;

    // NOTE: We could clamp instead of Err.
    // The TS version does that. Not sure which one is better.
    // We also could (should?) be checking for the color space
    if cx < 1 || cx > 9 || cy < 1 || cy > 9 {
        return Err(EncodingError::ComponentsNumberInvalid);
    }

    if width * height * 4 != pixels.len() {
        return Err(EncodingError::BytesPerPixelMismatch);
    }

    let mut dc: [f64; 3] = [0., 0., 0.];
    let mut ac: Vec<[f64; 3]> = Vec::with_capacity(cy * cx - 1);

    for y in 0..cy {
        for x in 0..cx {
            let normalisation = if x == 0 && y == 0 { 1f64 } else { 2f64 };
            let factor = multiply_basis_function(
                &pixels,
                width,
                height,
                bytes_per_row,
                bytes_per_pixel,
                0,
                |a, b| {
                    normalisation
                        * f64::cos((PI * x as f64 * a) / width as f64)
                        * f64::cos((PI * y as f64 * b) / height as f64)
                },
            );

            if x == 0 && y == 0 {
                // The first iteration is the dc
                dc = factor;
            } else {
                // All others are the ac
                ac.push(factor);
            }
        }
    }

    let mut hash = String::with_capacity(1 + 1 + 4 + 2 * ac.len());

    let size_flag = ((cx - 1) + (cy - 1) * 9) as usize;
    hash.extend(encode_base83_string(size_flag, 1));

    let maximum_value: f64;

    if !ac.is_empty() {
        // I'm sure there's a better way to write this; following the Swift atm :)
        let maxf = |a: &f64, b: &f64|  a.partial_cmp(b).unwrap_or(Ordering::Equal);
        let actual_maximum_value = ac
            .iter()
            .map(|channels| channels.iter().copied().map(f64::abs).max_by(maxf).unwrap())
            .max_by(maxf)
            .unwrap();
        let quantised_maximum_value = usize::max(
            0,
            usize::min(82, f64::floor(actual_maximum_value * 166f64 - 0.5) as usize),
        );
        maximum_value = ((quantised_maximum_value + 1) as f64) / 166f64;
        hash.extend(encode_base83_string(quantised_maximum_value, 1));
    } else {
        maximum_value = 1f64;
        hash.extend(encode_base83_string(0, 1));
    }

    hash.extend(encode_base83_string(encode_dc(dc), 4));

    for factor in ac {
        hash.extend(encode_base83_string(encode_ac(factor, maximum_value), 2));
    }

    Ok(hash)
}

fn multiply_basis_function<F>(
    pixels: &[u8],
    width: usize,
    height: usize,
    bytes_per_row: usize,
    bytes_per_pixel: usize,
    pixel_offset: usize,
    basis_function: F,
) -> [f64; 3]
where
    F: Fn(f64, f64) -> f64,
{
    let mut r = 0f64;
    let mut g = 0f64;
    let mut b = 0f64;

    for x in 0..width {
        for y in 0..height {
            let basis = basis_function(x as f64, y as f64);
            r += basis
                * srgb_to_linear(
                    usize::try_from(pixels[bytes_per_pixel * x + pixel_offset + y * bytes_per_row])
                        .unwrap(),
                );
            g += basis
                * srgb_to_linear(
                    usize::try_from(
                        pixels[bytes_per_pixel * x + pixel_offset + 1 + y * bytes_per_row],
                    )
                    .unwrap(),
                );
            b += basis
                * srgb_to_linear(
                    usize::try_from(
                        pixels[bytes_per_pixel * x + pixel_offset + 2 + y * bytes_per_row],
                    )
                    .unwrap(),
                );
        }
    }

    let scale = 1f64 / ((width * height) as f64);

    [r * scale, g * scale, b * scale]
}

fn encode_dc([r, g, b]: [f64; 3]) -> usize {
    let rounded = |v| linear_to_srgb(v);
    ((rounded(r) << 16) + (rounded(g) << 8) + rounded(b)) as usize
}

fn encode_ac([r, g, b]: [f64; 3], maximum_value: f64) -> usize {
    let quant = |v| {
        (sign_pow(v / maximum_value, 0.5) * 9. + 9.5)
            .floor()
            .min(18.)
            .max(0.)
    };

    (quant(r) * 19f64 * 19f64 + quant(g) * 19f64 + quant(b)) as usize
}

// Base83

// I considered using lazy_static! for this, but other implementations
// seem to hard-code these as well. Doing that for consistency.
static ENCODE_CHARACTERS: [char; 83] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '#', '$', '%', '*', '+', ',', '-', '.', ':', ';', '=', '?', '@', '[',
    ']', '^', '_', '{', '|', '}', '~',
];

fn decode_base83_string(string: &str) -> usize {
    string
        .chars()
        .filter_map(|character| ENCODE_CHARACTERS.iter().position(|&c| c == character))
        .fold(0, |value, digit| value * 83 + digit)
}

fn encode_base83_string(value: usize, length: u32) -> impl Iterator<Item = char> {
    (1..=length)
        .map(move |i| (value / 83usize.pow(length - i)) % 83)
        .map(|digit| ENCODE_CHARACTERS[digit])
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
