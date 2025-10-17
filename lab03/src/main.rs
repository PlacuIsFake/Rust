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

}
