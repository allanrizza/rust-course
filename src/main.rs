#[path = "../section2/src/section2.rs"] mod section2;
mod pause;
fn main() {
    section2::section_2_fn();

    // The pause method is used to prevent the program from closing immediately after executing the executable file. The pause method waits for user input before closing the terminal window, allowing enough time to read any output or error messages that may be displayed before the program exits.
    pause::pause();
}
