fn add(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > u32::MAX as u64 {
        panic!("x+y={x}+{y} is not a u32 value");
    }
    x + y
}
fn mul(x: u32, y: u32) -> u32 {
    let rez = x as u64 * y as u64;
    if rez > u32::MAX as u64 {
        panic!("x*y={x}*{y} is not a u32 value");
    }
    x * y
}
fn main() {
    println!("{}", add(15, 50));
    println!("{}", mul(2, 1024));
    //println!("{}", add(u32::MAX, u32::MAX));
    println!("{}", mul(u32::MAX, u32::MAX));
}
