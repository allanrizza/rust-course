pub fn slices() {
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v[2..4];

    println!("{:?}", sv);

    let string = String::from("hello world");

    println!("{}", &string[0..first_word(&string)]);
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}