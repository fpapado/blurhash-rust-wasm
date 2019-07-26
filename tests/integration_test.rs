use blurhash_wasm::decode;
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
    let res = decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);

    // From a known encode/decode
    let expected = image::open("decode-test-in.png").unwrap().to_rgba();

    match res {
        Ok(img) => {
            // image::save_buffer("decode-test-out.png", &img, 40, 30, image::RGBA(8));
            assert_eq!(expected.to_vec(), img);
        }

        Err(_err) => assert!(false),
    }
}

// TODO: encodes_ok (open file, encode, match)
// TODO: Round trip
