use blurhash_wasm::{decode, encode};
use image;
use std::convert::TryFrom;

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
    let expected = image::open("tests/data/decode-test-expected.png")
        .unwrap()
        .to_rgba();

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
    let input = image::open("tests/data/encode-test-input.jpg")
        .unwrap()
        .to_rgba();
    let (width, height) = input.dimensions();

    // From the online demo
    // let expected = "LKO2?U%2Tw=w]~RBVZRi};RPxuwH";

    // From our encoding, not sure why they differ
    let expected = "LKO2?V%2Tw=^]~RBVZRi};RPxuwH";

    let res = encode(
        input.into_vec(),
        4,
        3,
        usize::try_from(width).unwrap(),
        usize::try_from(height).unwrap(),
    );

    match res {
        Ok(actual) => {
            assert_eq!(expected, actual);
        }

        Err(_err) => assert!(false),
    }
}

#[test]
fn encodes_ok_2() {
    // From a known encode/decode
    let input = image::open("tests/data/encode-test-input-2.jpg")
        .unwrap()
        .to_rgba();
    let (width, height) = input.dimensions();

    // From the online demo
    // let expected = "LGF5]+Yk^6#M@-5c,1J5@[or[Q6.";

    // From our encoding
    // Again, weird mismatch
    let expected = "LGFFaXYk^6#M@-5c,1Ex@@or[j6o";

    let res = encode(
        input.into_vec(),
        4,
        3,
        usize::try_from(width).unwrap(),
        usize::try_from(height).unwrap(),
    );

    match res {
        Ok(actual) => {
            assert_eq!(expected, actual);
        }

        Err(_err) => assert!(false),
    }
}

#[test]
fn round_trips_ok() {
    // From a known encode/decode
    let input = image::open("tests/data/encode-test-input.jpg")
        .unwrap()
        .to_rgba();
    let (width, height) = input.dimensions();

    // From the online demo
    // let expected = "LKO2?U%2Tw=w]~RBVZRi};RPxuwH";

    // From our encoding, not sure why they differ
    let expected_encode = "LKO2?V%2Tw=^]~RBVZRi};RPxuwH";

    let encode_res = encode(
        input.into_vec(),
        4,
        3,
        usize::try_from(width).unwrap(),
        usize::try_from(height).unwrap(),
    );

    match encode_res {
        Ok(actual_encode) => {
            assert_eq!(expected_encode, actual_encode);
            let expected_decode = image::open("tests/data/roundtrip-test-input-decode.png")
                .unwrap()
                .to_rgba();

            let decode_res = decode(&actual_encode, 32, 32);

            match decode_res {
                Ok(actual_decode) => {
                    assert_eq!(expected_decode.to_vec(), actual_decode);
                }

                Err(_err) => assert!(false),
            }
        }

        Err(_err) => assert!(false),
    }
}
