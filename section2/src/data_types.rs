pub fn data_types() {
    let x: i8 = 10;
    println!("{}", x);

    let _y: u8 = 10; // underscore before the variable name will silent the warns
    
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}",decimal);
    println!("{}",hex);
    println!("{}",octal);
    println!("{}",binary);

    let byte = b'A';

    println!("{}",byte);

    let _x = 2.0; //f64 default because on moderns CPUs 
    let _y: f32 = 1.0;

    let _t = true;
    let _f: bool = false;

    let c = 'c';
    println!("{}", c);

    // + - * / %

    let a: f32 = 10.0;
    let b: f32 = 3.0;

    let remainder: f32 = a / b;

    println!("{:.2}", remainder);
}
