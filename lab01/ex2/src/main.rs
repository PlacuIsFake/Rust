fn is_coprime(mut a: i32, mut b: i32) -> bool {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a == 1
}
fn main() {
    let mut x = 0;
    while x <= 100 {
        let mut y = 0;
        while y <= 100 {
            if is_coprime(x, y) {
                println!("{x} and {y} are coprime");
            } else {
                println!("{x} and {y} are not coprime");
            }
            y += 1;
        }
        x += 1;
    }
}
