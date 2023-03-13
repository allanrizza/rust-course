#[path = "functions.rs"] mod functions;
pub fn section_2_fn() {
    functions::print_phrase("Print my argument");
    println!("{}", functions::gcd(20, 5));
    println!("{}", functions::multiple_return_values(true));
}
