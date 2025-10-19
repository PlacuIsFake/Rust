use std::io::Write;

use rand::seq::SliceRandom;

#[derive(Debug)]
enum Errors {
    Overflow,
    NotASCII,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
    Not2DigitNumber,
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
        Errors::Not2DigitNumber => println!("The string is not a whole number with 2 or less digits"),
    }
}

fn text_to_number(s: &mut str) -> Result<u8, Errors> {
    let mut nr = 0;
    if s.trim().len() > 2 {
        return Err(Errors::Not2DigitNumber);
    }
    for ch in s.trim().chars(){
        let d = char_to_number(ch)?;
        nr = nr * 10 + d;
    }
    Ok(nr)
}
fn init(d: &mut [(u8, char)]) {
    let mut k = 0;
    let mut i = 8;
    d[k].0 = 1;
    d[k].1 = 'S';
    k += 1;
    while i <= 13 {
        d[k].0 = i;
        d[k].1 = 'S';
        i += 1;
        k += 1;
    }
    i = 8;
    d[k].0 = 1;
    d[k].1 = 'H';
    k += 1;
    while i <= 13 {
        d[k].0 = i;
        d[k].1 = 'H';
        i += 1;
        k += 1;
    }
    i = 8;
    d[k].0 = 1;
    d[k].1 = 'C';
    k += 1;
    while i <= 13 {
        d[k].0 = i;
        d[k].1 = 'C';
        i += 1;
        k += 1;
    }
    i = 8;
    while i <= 12 {
        d[k].0 = i;
        d[k].1 = 'D';
        i += 1;
        k += 1;
    }
}
fn print_deck (symbols: bool) {
    if symbols {
        let mut i = 8;
        while i <= 10 {
            print!("{i}♠️  ");
            i += 1;
        }
        println!("J♠️  Q♠️  K♠️  A♠️");
        i = 8;
        while i <= 10 {
            print!("{i}♥️  ");
            i += 1;
        }
        println!("J♥️  Q♥️  K♥️  A♥️");
        i = 8;
        while i <= 10 {
            print!("{i}♣️  ");
            i += 1;
        }
        println!("J♣️  Q♣️  K♣️  A♣️");
        i = 8;
        while i <= 10 {
            print!("{i}♦️  ");
            i += 1;
        }
        println!("J♦️  Q♦️  K♦️");
    }
    else {
        let mut i = 8;
        while i <= 10 {
            print!("{i}S  ");
            i += 1;
        }
        println!("JS  QS  KS  AS");
        i = 8;
        while i <= 10 {
            print!("{i}H  ");
            i += 1;
        }
        println!("JH  QH  KH  AH");
        i = 8;
        while i <= 10 {
            print!("{i}C  ");
            i += 1;
        }
        println!("JC  QC  KC  AC");
        i = 8;
        while i <= 10 {
            print!("{i}D  ");
            i += 1;
        }
        println!("JD  QD  KD");
    }
}
fn print_deck_3rows(d: &[(u8, char)], symbols: bool) {
    let mut i = 0;
    print!("1: ");
    while i < 27 {
        match d[i].0 {
            1 => print!(" A"),
            8..=9 => print!(" {}", d[i].0),
            10 => print!("10"),
            11 => print!(" J"),
            12 => print!(" Q"),
            13 => print!(" K"),
            _ => {println!("\n{}", d[i].0); panic!("The deck is corupted")},
        }
        match d[i].1 {
            'S' => match symbols {
                true => print!("♠️  "),
                false => print!("S  "),
            }
            'H' => match symbols {
                true => print!("♥️  "),
                false => print!("H  "),
            }
            'C' => match symbols {
                true => print!("♣️  "),
                false => print!("C  "),
            }
            'D' => match symbols {
                true => print!("♦️  "),
                false => print!("D  "),
            }
            _ => {println!("\n{}", d[i].1); panic!("The deck is corupted")},
        }
        i += 3;
    }
    print!("\n2: ");
    i = 1;
    while i < 27 {
        match d[i].0 {
            1 => print!(" A"),
            8..=9 => print!(" {}", d[i].0),
            10 => print!("10"),
            11 => print!(" J"),
            12 => print!(" Q"),
            13 => print!(" K"),
            _ => {println!("\n{}", d[i].0); panic!("The deck is corupted")},
        }
        match d[i].1 {
            'S' => match symbols {
                true => print!("♠️  "),
                false => print!("S  "),
            }
            'H' => match symbols {
                true => print!("♥️  "),
                false => print!("H  "),
            }
            'C' => match symbols {
                true => print!("♣️  "),
                false => print!("C  "),
            }
            'D' => match symbols {
                true => print!("♦️  "),
                false => print!("D  "),
            }
            _ => {println!("\n{}", d[i].1); panic!("The deck is corupted")},
        }
        i += 3;
    }
    print!("\n3: ");
    i = 2;
    while i < 27 {
        match d[i].0 {
            1 => print!(" A"),
            8..=9 => print!(" {}", d[i].0),
            10 => print!("10"),
            11 => print!(" J"),
            12 => print!(" Q"),
            13 => print!(" K"),
            _ => {println!("\n{}", d[i].0); panic!("The deck is corupted")},
        }
        match d[i].1 {
            'S' => match symbols {
                true => print!("♠️  "),
                false => print!("S  "),
            }
            'H' => match symbols {
                true => print!("♥️  "),
                false => print!("H  "),
            }
            'C' => match symbols {
                true => print!("♣️  "),
                false => print!("C  "),
            }
            'D' => match symbols {
                true => print!("♦️  "),
                false => print!("D  "),
            }
            _ => {println!("\n{}", d[i].1); panic!("The deck is corupted")},
        }
        i += 3;
    }
    println!();
}
fn magic(mut nr: u8) -> (u8, u8, u8) {
    let nr1 = nr % 3;
    nr /= 3;
    let nr2 = nr % 3;
    nr /= 3;
    let nr3 = nr % 3;
    (nr1, nr2, nr3)
}
fn magic_shuffle(d: &mut [(u8, char)], row:u8, nr: u8) {
    let mut r1: [(u8, char); 9] = [(1, 'D'); 9];
    let mut r2: [(u8, char); 9] = [(1, 'D'); 9];
    let mut r3: [(u8, char); 9] = [(1, 'D'); 9];
    let mut i;
    let mut k;
    i = 0;
    k = 0;
    while i < 27 {
        r1[k] = d[i];
        r2[k] = d[i + 1];
        r3[k] = d[i + 2];
        i += 3;
        k += 1;
    }
    match row {
        1 => {
            match nr {
                0 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r1[i];
                        d[k + 9] = r2[i];
                        d[k + 18] = r3[i];
                        k += 1;
                        i += 1;
                    }
                }
                1 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r2[i];
                        d[k + 9] = r1[i];
                        d[k + 18] = r3[i];
                        k += 1;
                        i += 1;
                    }
                } 
                2 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r2[i];
                        d[k + 9] = r3[i];
                        d[k + 18] = r1[i];
                        k += 1;
                        i += 1;
                    }
                }
                _ => panic!("Magic number is not a base3 digit"),
            }
        }
        2 => {
            match nr {
                0 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r2[i];
                        d[k + 9] = r1[i];
                        d[k + 18] = r3[i];
                        k += 1;
                        i += 1;
                    }
                }
                1 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r3[i];
                        d[k + 9] = r2[i];
                        d[k + 18] = r1[i];
                        k += 1;
                        i += 1;
                    }
                } 
                2 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r3[i];
                        d[k + 9] = r1[i];
                        d[k + 18] = r2[i];
                        k += 1;
                        i += 1;
                    }
                }
                _ => panic!("Magic number is not a base3 digit"),
            }
        }
        3 => {
            match nr {
                0 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r3[i];
                        d[k + 9] = r2[i];
                        d[k + 18] = r1[i];
                        k += 1;
                        i += 1;
                    }
                }
                1 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r1[i];
                        d[k + 9] = r3[i];
                        d[k + 18] = r2[i];
                        k += 1;
                        i += 1;
                    }
                } 
                2 => {
                    let mut k = 0;
                    let mut i = 0;
                    while i < 9 {
                        d[k] = r1[i];
                        d[k + 9] = r2[i];
                        d[k + 18] = r3[i];
                        k += 1;
                        i += 1;
                    }
                }
                _ => panic!("Magic number is not a base3 digit"),
            }
        }
        _ => panic!("Row out of bounds"),
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

    println!("\nEx5:");
    let mut nr = 0;
    
    while nr == 0 {
        let mut input = String::from("");
        print!("Choose a number between 1 and 27 (a whole number): ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        match input.trim_end() {
            "" => {println!("If you're not goning to pick a number, I'll pick one for you."); nr = 1;},
            _ => {
                    match text_to_number(&mut input) {
                        Ok(x) => {nr = x; if nr == 0 || nr > 27 {println!("Number is not between 1 and 27\n Let's try again"); nr = 0}},
                        Err(err) => {print_error(err); println!("Let's try again.")
                    }
                }
            }
        }
    }
    let rounds = magic(nr - 1);
    println!("Now pick a card from this deck. Any card.");
    let mut symbols = true;
    print_deck(symbols);
    println!("Do you see the symbols? (y/n/q)");
    let mut input = String::from("");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    match input.trim_end() {
        "y" => println!("Perfect!"),
        "Y" => println!("Perfect!"),
        "n" => {
            println!("Then I will use this deck.");
            symbols = false;
            print_deck(symbols);
        }
        "N" => {
            println!("Then I will use this deck.");
            symbols = false;
            print_deck(symbols);
        }
        "q" => return,
        "Q" => return,
        _ => {
            println!("I'll take that as a no. We'll use this deck instead");
            symbols = false;
            print_deck(symbols);
        }
    }
    println!("Did you pick a card? (y/n/q)");
    input = String::from("");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    match input.trim_end() {
        "y" => println!("Perfect!"),
        "Y" => println!("Perfect!"),
        "d" => println!("Perfect!"),
        "D" => println!("Perfect!"), 
        _ => return,
    }
    println!("Now I'm going to shuffle the deck and place the cards in 3 rows.");
    let mut deck : [(u8, char); 27] = [(1, 'D'); 27];
    let mut rng = rand::rng();
    init(&mut deck);
    deck.shuffle(&mut rng);
    print_deck_3rows(&deck, symbols);
    println!("In which row is your card? (1/2/3/q)");
    input = String::from("");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    match input.trim_end() {
        "1" => magic_shuffle(&mut deck, 1, rounds.0),
        "2" => magic_shuffle(&mut deck, 2, rounds.0),
        "3" => magic_shuffle(&mut deck, 3, rounds.0),
        _ => return,
    }
    println!("We're gonna do this 2 more times");
    print_deck_3rows(&deck, symbols);
    println!("In which row is your card? (1/2/3/q)");
    input = String::from("");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    match input.trim_end() {
        "1" => magic_shuffle(&mut deck, 1, rounds.1),
        "2" => magic_shuffle(&mut deck, 2, rounds.1),
        "3" => magic_shuffle(&mut deck, 3, rounds.1),
        _ => return,
    }
    println!("Last time");
    print_deck_3rows(&deck, symbols);
    println!("In which row is your card? (1/2/3/q)");
    input = String::from("");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    match input.trim_end() {
        "1" => magic_shuffle(&mut deck, 1, rounds.2),
        "2" => magic_shuffle(&mut deck, 2, rounds.2),
        "3" => magic_shuffle(&mut deck, 3, rounds.2),
        _ => return,
    }
    println!("Is this your card? (y/n/q)");
    nr -= 1;
    match deck[nr as usize].0 {
        1 => print!(" A"),
        8..=9 => print!(" {}", deck[nr as usize].0),
        10 => print!("10"),
        11 => print!(" J"),
        12 => print!(" Q"),
        13 => print!(" K"),
        _ => {println!("\n{}", deck[nr as usize].0); panic!("The deck is corupted")},
    }
    match deck[nr as usize].1 {
        'S' => match symbols {
            true => print!("♠️  "),
            false => print!("S  "),
        }
        'H' => match symbols {
            true => print!("♥️  "),
            false => print!("H  "),
        }
        'C' => match symbols {
            true => print!("♣️  "),
            false => print!("C  "),
        }
        'D' => match symbols {
            true => print!("♦️  "),
            false => print!("D  "),
        }
        _ => {println!("\n{}", deck[nr as usize].1); panic!("The deck is corupted")},
    }
    println!();
    input = String::from("");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    match input.trim_end() {
        "y" => println!("Thank you for your attention"),
        "Y" => println!("Thank you for your attention"),
        "n" => panic!("It did not work somehow"),
        "N" => panic!("It did not work somehow"),
        _ => (),
    }
}

    