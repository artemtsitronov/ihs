fn main() {
    let mut img = image::open("Cat.png").unwrap().to_rgb8();

    let p = img.get_pixel(0, 0);
    println!("before: {}, {}, {}", p[0], p[1], p[2]);

    let p = img.get_pixel_mut(0, 0);
    p[0] = (p[0] & 0xFE) | 1;
    p[1] = (p[1] & 0xFE) | 0;
    p[2] = (p[2] & 0xFE) | 1;

    img.save("eed_out.png").unwrap();

    let img2 = image::open("eed_out.png").unwrap().to_rgb8();
    let p = img2.get_pixel(0, 0);

    println!("afterr: {} {} {}", p[0], p[1], p[2]);
    println!("last bit: {} {} {}", p[0] & 1, p[1] & 1, p[2] & 1);
}
