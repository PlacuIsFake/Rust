fn add_space(s: &mut String, n: i32) {
    let mut i = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}
fn add_str(s: &mut String, str: String) {
    s.push_str(&str);
}
fn add_integer(s: &mut String, mut n: i32) {
    let mut ogl = 0;
    let mut zero_in_back = 0;
    let mut nr_cif = 0;
    let mut add_zero = true;
    while n > 0 {
        let cif = n % 10;
        if cif == 0 && add_zero {
            zero_in_back += 1;
        } else {
            add_zero = false;
        }
        ogl = ogl * 10 + cif;
        nr_cif += 1;
        n /= 10;
    }
    if nr_cif % 3 == 1 && nr_cif != 1 {
        s.push(((ogl % 10) as u8 + b'0') as char);
        ogl /= 10;
        s.push('_');
    } else if nr_cif % 3 == 2 && nr_cif != 2 {
        s.push(((ogl % 10) as u8 + b'0') as char);
        ogl /= 10;
        s.push(((ogl % 10) as u8 + b'0') as char);
        ogl /= 10;
        s.push('_');
    }
    let mut k = 0;
    while ogl > 0 {
        if k == 3 {
            s.push('_');
            k = 0;
        }
        let cif = ogl % 10;
        s.push((cif as u8 + b'0') as char);
        ogl /= 10;
        k += 1;
    }
    while zero_in_back > 0 {
        if k == 3 {
            s.push('_');
            k = 0;
        }
        s.push('0');
        k += 1;
        zero_in_back -= 1;
    }
}
fn add_float(s: &mut String, n: f64) {
    let mut copy = n;
    let mut nr_cif_before_dot = 0;
    while copy >= 1.0 {
        copy /= 10.0;
        nr_cif_before_dot += 1;
    }
    let mut i = 0;
    while copy > 0.0 && i <= 3 {
        copy *= 10.0;
        let cif = (copy as i32) % 10;
        copy -= cif as f64;
        nr_cif_before_dot -= 1;
        s.push((cif as u8 + b'0') as char);
        if nr_cif_before_dot == 0 {
            s.push('.');
        }
        if nr_cif_before_dot <= 0 {
            i += 1;
        }
    }
}
fn main() {
    let mut s = String::from("");
    let mut_ref_to_s = &mut s;
    add_space(mut_ref_to_s, 40);
    add_str(mut_ref_to_s, String::from("I"));
    add_space(mut_ref_to_s, 1);
    add_str(mut_ref_to_s, String::from("💚\n"));
    add_space(mut_ref_to_s, 40);
    add_str(mut_ref_to_s, String::from("RUST\n\n"));
    add_space(mut_ref_to_s, 4);
    add_str(mut_ref_to_s, String::from("Most"));
    add_space(mut_ref_to_s, 12);
    add_str(mut_ref_to_s, String::from("crate"));
    add_space(mut_ref_to_s, 6);
    add_integer(mut_ref_to_s, 306437968);
    add_space(mut_ref_to_s, 11);
    add_str(mut_ref_to_s, String::from("and"));
    add_space(mut_ref_to_s, 5);
    add_str(mut_ref_to_s, String::from("lastest"));
    add_space(mut_ref_to_s, 9);
    add_str(mut_ref_to_s, String::from("is\n"));
    add_space(mut_ref_to_s, 9);
    add_str(mut_ref_to_s, String::from("downloaded"));
    add_space(mut_ref_to_s, 8);
    add_str(mut_ref_to_s, String::from("has"));
    add_space(mut_ref_to_s, 13);
    add_str(mut_ref_to_s, String::from("downloads"));
    add_space(mut_ref_to_s, 5);
    add_str(mut_ref_to_s, String::from("the"));
    add_space(mut_ref_to_s, 9);
    add_str(mut_ref_to_s, String::from("version"));
    add_space(mut_ref_to_s, 4);
    add_float(mut_ref_to_s, 2.0389); //am adaugat o cifra dupa virgula pentru a afisa 2.038 dupa executie
    print!("{s}");
}
