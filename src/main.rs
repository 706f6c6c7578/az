use std::io::{self, Read, Write};
use std::env;
use std::process;

fn encode(input: u8, second_channel: bool) -> char {
    match input {
        0..=9 if second_channel => (input + b'A') as char,
        0..=9 => (input + b'Q') as char,
        10..=15 => (input - 10 + b'K') as char,
        _ => panic!("Invalid input"),
    }
}

fn decode(input: char) -> (u8, bool) {
    match input {
        'A'..='J' => (input as u8 - b'A', false),
        'Q'..='Z' => (input as u8 - b'Q', true),
        'K'..='P' => (input as u8 - b'K' + 10, false),
        _ => panic!("Invalid input"),
    }
}

fn encode_binary(input: &[u8], line_length: usize) -> String {
    let mut result = String::new();
    let mut char_count = 0;
    let mut second_channel = false;
    for &byte in input.iter() {
        if char_count != 0 && char_count % line_length == 0 {
            result.push('\n');
        }
        let high = byte >> 4;
        let low = byte & 0x0F;
        let encoded_high = encode(high, second_channel);
        let encoded_low = encode(low, !second_channel);
        result.push(encoded_high);
        result.push(encoded_low);
        second_channel = !second_channel;
        char_count += 2;
    }
    result
}

fn decode_binary(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut chars = input.chars().filter(|&c| c != '\n');
    while let Some(high) = chars.next() {
        if let Some(low) = chars.next() {
            let (decoded_high, _) = decode(high);
            let (decoded_low, _) = decode(low);
            result.push(decoded_high << 4 | decoded_low);
        }
    }
    result
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: az [-d] [-l line length]");
    process::exit(1);
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let mut decode_flag = false;
    let mut line_length = usize::MAX;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => decode_flag = true,
            "-l" => match args.next().and_then(|s| s.parse().ok()) {
                Some(n) => {
                    if n % 2 == 0 {
                        line_length = n;
                    } else {
                        eprintln!("Odd line length is not allowed. Please enter an even number.");
                        process::exit(1);
                    }
                },
                None => print_usage_and_exit(),
            },
            _ => print_usage_and_exit(),
        }
    }

    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    if decode_flag {
        let input = String::from_utf8(buffer).unwrap();
        let decoded = decode_binary(&input);
        io::stdout().write_all(&decoded)?;
    } else {
        let encoded = encode_binary(&buffer, line_length);
        io::stdout().write_all(encoded.as_bytes())?;
        if !decode_flag {
            println!();
        }
    }

    Ok(())
}
