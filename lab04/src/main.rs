use std::{fs, io};
fn read_file(path: &String) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
    Ok(s)
}
fn count_bytes(s: &str) -> i32
{
    let mut cnt = 0;
    for _b in s.bytes() {
        cnt += 1;
    }
    cnt
}
fn count_chars(s: &str) -> i32 {
    let mut cnt = 0;
    for _c in s.chars() {
        cnt += 1;
    }
    cnt
}

fn rot13_cipher(s: &str) -> Result<String, &'static str> {
    let mut new_s = String::from("");
    for c in s.chars() {
        if !c.is_ascii() {
            return Err("Character is not ascii");
        }
        let mut new_c = '?';
        match c {
            'a'..='z' => new_c = (((c as u8 - b'a') + 13) % 26 + b'a') as char,
            'A'..='Z' => new_c = (((c as u8 - b'A') + 13) % 26 + b'A') as char,
            _ => {},
        }
        new_s.push(new_c);
    }
    Ok(new_s)
}

fn main() {
    println!("Ex1:");
    let path = String::from("Text1.txt");
    let mut file_cont = String::from("");
    match read_file(&path) {
        Ok(x) => file_cont = x,
        Err(e) => println!("{e:?}"),
    }
    let s = file_cont.as_str();
    let mut b_max = 0;
    let mut c_max = 0;
    let str_init = String::from("");
    let mut most_bytes_line = str_init.as_str();
    let mut most_chars_line = str_init.as_str();
    for l in s.lines() {
        let b = count_bytes(l);
        let c = count_chars(l);
        if b_max < b {
            b_max = b;
            most_bytes_line = l;
        }
        if c_max < c {
            c_max = c;
            most_chars_line = l;
        }
    }
    println!("The longest line considering the number of bytes is : {most_bytes_line}");
    println!("The longest line considering the number of actual characters is : {most_chars_line}");
    println!("Ex2:");
    let s = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    match rot13_cipher(s.as_str()) {
        Ok(new_s) => println!("{new_s}"),
        Err(e) => println!("{e}"),
    }
}
