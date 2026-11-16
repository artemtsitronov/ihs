const START: &[u8] = b"EED<";
const END: &[u8] = b">EED";

fn encrypt(image: &mut [u8], data: &str) {
    let bytes: &[u8] = data.as_bytes();
    for (i, byte) in [START, bytes, END].concat().iter().enumerate() {
        for bit in 0..8 {
            let b = (byte >> (7 - bit)) & 1;
            let idx = i * 8 + bit;
            image[idx] = (image[idx] & 0xFE) | b;
        }
    }
}

fn decrypt(img: &mut [u8]) -> Option<String> {
    let read_byte = |img: &mut [u8], i: usize| {
        let mut got: u8 = 0;
        for bit in 0..8 {
            got = (got << 1) | img[i * 8 + bit] & 1;
        }
        got
    };

    let total = img.len() / 8;
    if total < START.len() {
        return None;
    }

    for (i, expected) in START.iter().enumerate() {
        if read_byte(img, i) != *expected {
            return None;
        }
    }

    let mut buf: Vec<u8> = Vec::new();
    for i in START.len()..total {
        buf.push(read_byte(img, i));
        if buf.ends_with(END) {
            buf.truncate(buf.len() - END.len()); // cut it
            return String::from_utf8(buf).ok();
        }
    }

    None
}

fn main() {
    let mut img = image::open("Cat.png").unwrap().to_rgb8();
    encrypt(&mut img, "Hello worlddd!");
    img.save("eed_out.png").unwrap();

    // now lets read ts

    let mut img2 = image::open("eed_out.png").unwrap().to_rgb8();

    if let Some(decrypted) = decrypt(&mut img2) {
        println!("{}", decrypted);
    };
}
