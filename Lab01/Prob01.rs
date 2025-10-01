fn este_prim(x: i32) -> bool {
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
    return true;
}
fn main() {
    let mut x = 0;
    while x <= 100 {
        if este_prim(x) {
            print!("{} este prim!\n", x);
        } else {
            print!("{} nu este prim!\n", x);
        }
        x += 1;
    }
}
