#[derive(Debug)]
enum Errors {
    Overflow,
    NotASCII,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
fn next_prime(x: u16) -> Option<u16> {
    let mut nxt: u32 = (x as u32) + 1;
    while nxt < u16::MAX as u32 {
        if is_prime(nxt) {
            return Some(nxt as u16);
        }
        nxt += 1;
    }
    None
}

fn add_panic(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > u32::MAX as u64 {
        panic!("x+y={x}+{y} is not a u32 value");
    }
    x + y
}
fn mul_panic(x: u32, y: u32) -> u32 {
    let rez = x as u64 * y as u64;
    if rez > u32::MAX as u64 {
        panic!("x*y={x}*{y} is not a u32 value");
    }
    x * y
}

fn add(x: u32, y: u32) -> Result<u32, Errors> {
    let sum = x as u64 + y as u64;
    if sum > u32::MAX as u64 {
        return Err(Errors::Overflow);
    }
    Ok(x + y)
}
fn mul(x:u32, y: u32) -> Result<u32, Errors> {
    let rez = x as u64 * y as u64;
    if rez > u32::MAX as u64 {
        return Err(Errors::Overflow);
    }
    Ok(x * y)
}
fn add_n_mul(x:u32, y: u32) -> Result<u32, Errors> {
    let sum = add(x, y)?;
    let rez = mul(sum, sum)?;
    Ok(rez)
}

fn to_uppercase(ch: char) -> Result<char, Errors> {
    match ch {
        'a'..='z' => Ok(((ch as u8) - b'a' + b'A') as char),
        'A'..='Z' => Ok(ch),
        _ => Err(Errors::NotLetter),
    }
}
fn to_lowercase(ch: char) -> Result<char, Errors> {
    match ch {
        'a'..='z' => Ok(ch),
        'A'..='Z' => Ok(((ch as u8) - b'A' + b'a') as char),
        _ => Err(Errors::NotLetter),
    }
}
fn print_char(ch: char) -> Result<char, Errors> {
    if ch.is_ascii_graphic() {
        return Ok(ch);
    }
    Err(Errors::NotPrintable)
}
fn char_to_number(ch: char) -> Result<u8, Errors> {
    if !ch.is_ascii() {
        return Err(Errors::NotASCII);
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        _ => Err(Errors::NotDigit),
    }
}
fn char_to_number_hex(ch: char) -> Result<u8, Errors> {
    if !ch.is_ascii() {
        return Err(Errors::NotASCII);
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        'A'..='F' => Ok(ch as u8 - b'A' + 10),
        _ => Err(Errors::NotBase16Digit),
    }
}
fn print_error(err: Errors) {
    match err {
        Errors::NotASCII => println!("Character is not ascii"),
        Errors::NotDigit => println!("Character is not a digit"),
        Errors::NotBase16Digit => println!("Character is not a base16 digit"),
        Errors::NotLetter => println!("Character is not a letter"),
        Errors::NotPrintable => println!("Character is not printable"),
        Errors::Overflow => println!("Operation not possible in u32 : Overflow"),
    }
}

fn main() {
    println!("Ex1:");
    let mut x = 1;
    while let Some(v) = next_prime(x) {
        println!("{v}");
        x = v;
    }

    println!("\nEx2:");
    let x = 15;
    let y = 50;
    let z = u32::MAX;
    let r = std::panic::catch_unwind(|| add_panic(x, y));
    match r {
        Ok(sum) => println!("{x}+{y}={sum:?}"),
        Err(_) => println!("{x}+{y} is not a u32 value"),
    }
    let r = std::panic::catch_unwind(|| add_panic(z, z));
    match r {
        Ok(sum) => println!("{z}+{z}={sum:?}"),
        Err(_) => println!("{z}+{z} is not a u32 value"),
    }
    let r = std::panic::catch_unwind(|| mul_panic(x, y));
    match r {
        Ok(rez) => println!("{x}*{y}={rez:?}"),
        Err(_) => println!("{x}*{y} is not a u32 value"),
    }
    let r = std::panic::catch_unwind(|| mul_panic(z, z));
    match r {
        Ok(rez) => println!("{z}*{z}={rez:?}"),
        Err(_) => println!("{z}*{z} is not a u32 value"),
    }

    println!("\nEx3:");
    let x = 15;
    let y = 50;
    let z = u32::MAX;
    match add(x, y) {
        Ok(sum) => println!("{x}+{y}={sum}"),
        Err(err) => println!("{x}+{y} is not possible in u32 : {err:?}"),
    }
    match add(z, z) {
        Ok(sum) => println!("{z}+{z}={sum}"),
        Err(err) => println!("{z}+{z} is not possible in u32 : {err:?}"),
    }
    match mul(x, y) {
        Ok(rez) => println!("{x}*{y}={rez}"),
        Err(err) => println!("{x}*{y} is not possible in u32 : {err:?}"),
    }
    match mul(z, z) {
        Ok(rez) => println!("{z}*{z}={rez}"),
        Err(err) => println!("{z}*{z} is not possible in u32 : {err:?}"),
    }
    match add_n_mul(x, y) {
        Ok(rez) => println!("({x}+{y})*({x}+{y})={rez}"),
        Err(err) => println!("({x}+{y})*({x}+{y}) is not possible in u32 : {err:?}"),
    }
    match add_n_mul(x, z) {
        Ok(rez) => println!("({x}+{z})*({x}+{z})={rez}"),
        Err(err) => println!("({x}+{z})*({x}+{z}) is not possible in u32 : {err:?}"),
    }
    let z = u32::MAX - x;
    match add_n_mul(x, z) {
        Ok(rez) => println!("({x}+{z})*({x}+{z})={rez}"),
        Err(err) => println!("({x}+{z})*({x}+{z}) is not possible in u32 : {err:?}"),
    }

    println!("\nEx4:");
    match to_uppercase('a') {
        Ok(ch) => println!("Uppercase: 'a' -> '{ch}'"),
        Err(err) => {print!("Uppercase: ('a') "); print_error(err);},
    }
    match to_uppercase('B') {
        Ok(ch) => println!("Uppercase: 'B' -> '{ch}'"),
        Err(err) => {print!("Uppercase: ('B') "); print_error(err);},
    }
    match to_uppercase('6') {
        Ok(ch) => println!("Uppercase: '6' -> '{ch}'"),
        Err(err) => {print!("Uppercase: ('6') "); print_error(err);},
    }
    match to_lowercase('C') {
        Ok(ch) => println!("Lowercase: 'C' -> '{ch}'"),
        Err(err) => {print!("Lowercase: ('C') "); print_error(err);},
    }
    match to_lowercase('d') {
        Ok(ch) => println!("Lowercase: 'd' -> '{ch}'"),
        Err(err) => {print!("Lowercase: ('d') "); print_error(err);},
    }
    match to_lowercase('.') {
        Ok(ch) => println!("Lowercase: '.' -> '{ch}'"),
        Err(err) => {print!("Lowercase: ('.') "); print_error(err);},
    }
    match print_char('s') {
        Ok(ch) => println!("{ch}"),
        Err(err) => print_error(err),
    }
    match print_char('\n') {
        Ok(ch) => println!("{ch}"),
        Err(err) => print_error(err),
    }
    match char_to_number('9') {
        Ok(x) => println!("To number: '9' -> {x}"),
        Err(err) => {print!("To number: ('9') "); print_error(err);},
    }
    match char_to_number('b') {
        Ok(x) => println!("To number: 'b' -> {x}"),
        Err(err) => {print!("To number: ('b') "); print_error(err);},
    }
    match char_to_number('🦀') {
        Ok(x) => println!("To number: '🦀' -> {x}"),
        Err(err) => {print!("To number: ('🦀') "); print_error(err);},
    }
    match char_to_number_hex('0') {
        Ok(x) => println!("To Number Hex: '0' -> {x:X}"),
        Err(err) => {print!("To Number Hex: ('0') "); print_error(err);},
    }
    match char_to_number_hex('D') {
        Ok(x) => println!("To Number Hex: 'D' -> {x:X}"),
        Err(err) => {print!("To Number Hex: ('D') "); print_error(err);},
    }
    match char_to_number_hex('G') {
        Ok(x) => println!("To Number Hex: 'G' -> {x:X}"),
        Err(err) => {print!("To Number Hex: ('a') "); print_error(err);},
    }
    match char_to_number_hex('🦀') {
        Ok(x) => println!("To Number Hex: '🦀' -> {x:X}"),
        Err(err) => {print!("To Number Hex: ('🦀') "); print_error(err);},
    }
}
