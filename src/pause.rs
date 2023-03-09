use std::io::{stdin, stdout, Read, Write};

pub fn pause() {
    let mut stdout = stdout();
    stout.write(b"Press ENTER to continue . . .").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}