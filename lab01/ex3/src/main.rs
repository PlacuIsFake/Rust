fn main() {
    let mut x = 99;
    while x >= 0 {
        if x == 0 {
            println!(
                "No bottles of beer on the wall,\nNo bottles of beer.\nGo to the store, buy some more,\n99 bottles of beer on the wall.\n"
            );
        } else if x == 1 {
            println!(
                "1 bottle of beer on the wall,\n1 bottle of beer.\nTake one down, pass it around,\nNo bottles of beer on the wall.\n"
            );
        } else if x == 2 {
            println!(
                "2 bottles of beer on the wall,\n2 bottles of beer.\nTake one down, pass it around,\n1 bottle of beer on the wall.\n"
            );
        } else {
            println!(
                "{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottles of beer on the wall.\n",
                x,
                x,
                x - 1
            );
        }
        x -= 1;
    }
}
