use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "eed", about = "eed encrypts data: encrypt text inside images")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encrypt {
        input: String,
        text: String,

        #[arg(short, long, default_value = "eed_out.png")]
        output: String,
    },
    Decrypt {
        input: String,
    },
}

// -- //

const START: &[u8] = b"EED<";
const END: &[u8] = b">EED";

fn encrypt(image: &mut [u8], data: &str) -> Result<(), String> {
    let packed = [START, data.as_bytes(), END].concat();
    if packed.len() * 8 > image.len() {
        return Err(format!(
            "Too big for this image! Bytes: {}, image: {}",
            packed.len(),
            image.len() / 8,
        ));
    }

    for (i, byte) in packed.iter().enumerate() {
        for bit in 0..8 {
            let b = (byte >> (7 - bit)) & 1;
            let idx = i * 8 + bit;
            image[idx] = (image[idx] & 0xFE) | b;
        }
    }
    Ok(())
}

fn decrypt(img: &[u8]) -> Option<String> {
    let read_byte = |img: &[u8], i: usize| {
        let mut got: u8 = 0;
        for bit in 0..8 {
            got = (got << 1) | (img[i * 8 + bit] & 1);
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
    println!("EED - Eed Encrypts Data. Made in Rust by Artem Tsitronov.");

    let cli = Cli::parse();
    match cli.command {
        Command::Encrypt {
            input,
            text,
            output,
        } => {
            let mut image = image::open(input).expect("No image found").to_rgba8();
            if let Err(e) = encrypt(&mut image, &text) {
                eprintln!("Error - {}", e);
                std::process::exit(1);
            }

            image.save(&output).ok();
            println!("Ecryption was succesful. Saved to {}", output);
        }
        Command::Decrypt { input } => {
            let image = image::open(input).expect("No image found").to_rgba8();
            match decrypt(&image) {
                Some(decrypted) => println!("{}", decrypted),
                None => println!("No EED data found"),
            }
        }
    }
}
