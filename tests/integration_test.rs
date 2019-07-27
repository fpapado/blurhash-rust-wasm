use blurhash_wasm::{decode, encode};
use image;

#[test]
fn err_if_hash_length_less_than_6() {
    assert_eq!(
        Err(blurhash_wasm::Error::LengthInvalid),
        decode("L", 40, 30)
    );
}

#[test]
fn decodes_ok() {
    // From the online demo
    let res = decode("LUDT3yayV?ay%jWBa#a}9Xj[j@fP", 40, 30);

    // From a known encode/decode
    let expected = image::open("decode-test-expected.png").unwrap().to_rgba();

    match res {
        Ok(img) => {
            // image::save_buffer("decode-test-out.png", &img, 40, 30, image::RGBA(8));
            assert_eq!(expected.to_vec(), img);
        }

        Err(_err) => assert!(false),
    }
}

#[test]
fn encodes_ok() {
    // From a known encode/decode
    let input = image::open("encode-test-input.jpg").unwrap().to_rgba();
    let (width, height) = input.dimensions();

    // From the online demo
    let expected = "LUDT3yayV?ay%jWBa#a}9Xj[j@fP";

    // TODO: Think about argument order here...
    // What is more common in Rust? Data or config first?
    let res = encode(input.into_vec(), 4, 3, width, height);

    match res {
        Ok(img) => {
            assert_eq!(expected, img);
        }

        Err(_err) => assert!(false),
    }
}

// TODO: Round trip
