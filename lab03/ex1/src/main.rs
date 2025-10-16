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
    while nxt < 65535 {
        if is_prime(nxt) {
            return Some(nxt as u16);
        }
        nxt += 1;
    }
    None
}
fn main() {
    let mut x = 1;
    while let Some(v) = next_prime(x) {
        println!("{v}");
        x = v;
    }
}
