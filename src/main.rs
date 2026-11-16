fn main() {
    let mut img = image::open("Cat.png").unwrap().to_rgb8();

    let samples: &mut [u8] = &mut img;

    let byte: u8 = b'E';

    for bit in 0..8 {
        let b = (byte >> (7 - bit)) & 1;
        samples[bit] = (samples[bit] & 0xFE) | b; // oh yeah
    }

    img.save("eed_out.png").unwrap();

    // now lets read ts
    let mut img2 = image::open("eed_out.png").unwrap().to_rgb8();

    let samples2: &mut [u8] = &mut img2;

    let mut got: u8 = 0;
    for bit in 0..8 {
        got = (got << 1) | (samples2[bit] & 1);
    }

    println!("hidin': {}     read: {} ({})", byte, got, got as char)
}
