pub fn strings() {
    let name = String::from("Allan");
    let course = "Rust ".to_string();

    let new_name = name.replace("Allan", "Tyler");
    println!("{}", name);
    println!("{}", course);
    println!("{}\n", new_name);

    // &str - "string slice" or "stir"
    let str1 = "hello"; // &str
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str2);
    println!("{}", str2);
    println!("{}\n", str3);

    // compare string == != (does not equal)
    println!("{}", "ONE".to_lowercase() == "one");


    // EXPLORING STRING FUNCTIONS:
    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", &hello[..5]);
}