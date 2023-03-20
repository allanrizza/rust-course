pub fn print_slices() {
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v[2..4];
    println!("{:?}", sv);

    let string = String::from("hello world");
    let index = find_first_word_index(&string);
    println!("{}", &string[..index]);
}

fn find_first_word_index(s: &str) -> usize {
    s.find(' ').unwrap_or_else(|| s.len())
}
