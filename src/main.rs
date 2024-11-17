use std::{
    io::{self, Read},
    u32,
};

fn convert_group(mut group: u32, num_bytes: i32) {
    for _ in 0..num_bytes {
        let c: u8 = ((group & (u32::MAX.checked_shl(26).unwrap_or(0))) >> 26)
            .try_into()
            .unwrap();

        if c <= 25 {
            print!("{}", (c + b'A') as char);
        } else if c <= 51 {
            print!("{}", (c - 26 + b'a') as char);
        } else if c <= 61 {
            print!("{}", (c - 52 + b'0') as char);
        } else if c == 62 {
            print!("{}", '+');
        } else {
            print!("{}", '/');
        }

        group = group.checked_shl(6).unwrap_or(0);
    }
}

fn main() {
    // Read bytes from stdin
    let bytes: Vec<u8> = io::stdin().bytes().map(|input| input.unwrap()).collect();

    let mut i = 0;

    // Convert each group to base64 char
    while i + 2 < bytes.len() {
        let group: u32 = ((u32::from(bytes[i]) << 16)
            | (u32::from(bytes[i + 1]) << 8)
            | u32::from(bytes[i + 2]))
        .checked_shl(8)
        .unwrap_or(0);

        convert_group(group, 4);

        i += 3;
    }

    // Padding
    if bytes.len() % 3 == 2 {
        let group: u32 = ((u32::from(bytes[bytes.len() - 2]) << 16)
            | (u32::from(bytes[bytes.len() - 1]) << 8))
            .checked_shl(8)
            .unwrap_or(0);

        convert_group(group, 3);

        print!("=");
    } else if bytes.len() % 3 == 1 {
        let group: u32 = (u32::from(bytes[bytes.len() - 1]) << 16)
            .checked_shl(8)
            .unwrap_or(0);

        convert_group(group, 2);

        print!("==");
    }

    println!();
}
