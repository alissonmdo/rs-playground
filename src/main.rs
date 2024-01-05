mod programs;
use std::env;

use console::Term;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[1];
    Term::stdout().clear_screen().expect("Failed to clear to screen");
    if program == "hello" || program == "1" {
        programs::hello::main();
    } else if program == "guess" || program == "2" {
        programs::guess::main();
    } else {
        panic!("No program was provided")
    }
}
