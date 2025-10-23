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
fn main() {
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
}
