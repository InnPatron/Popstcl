extern crate popstcl_core;

use popstcl_core::vm::basic_vm;

use std::io;
use std::io::Write;

fn main() {
    let mut vm = basic_vm();
    greetings();
    loop {
        let mut program = String::new();
        io::stdin().read_line(&mut program).expect("Failed to read line");

        match &*program.trim() {
            "q" => break,
            "cls" => { 
                if let Err(e) = io::stdout().flush() {
                    panic!("Nonrecoverable Error\n{:?}", e);
                }
            },
            _ => (),
        }

        match vm.eval_string(&program) {
            Ok(_) => (),
            Err(e) => println!("Execution Error: {:?}", e),
        }
    }
}

fn greetings() {
    println!("Popstcl REPL TEST BUILD");
}
