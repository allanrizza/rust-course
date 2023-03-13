pub fn print_phrase(phrase: &str) {
    println!("{}", phrase);
}

pub fn gcd(
    mut a: u64,
    mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a%b;
    }
    b
}

pub fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}