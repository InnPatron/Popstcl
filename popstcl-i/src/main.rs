extern crate popstcl_core;
extern crate rustyline;

use rustyline::Editor;
use rustyline::completion::FilenameCompleter;

use popstcl_core::vm::basic_vm;

use std::io;
use std::io::Write;

fn main() {
    let mut vm = basic_vm();
    let mut editor = Editor::<()>::new();
    greetings();
    loop {
        let input = match editor.readline(">>> ") {
            Ok(str) => str,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        };
        let input = &*input.trim();
        match input {
            "q" => break,
            "cls" => { 
                clear();
                greetings();
                continue;
            },
            _ => (),
        }

        match vm.eval_string(input) {
            Ok(_) => (),
            Err(e) => println!("Execution Error: {:?}", e),
        }
    }
}

fn greetings() {
    println!("Popstcl REPL TEST BUILD");
}

#[cfg(windows)]
fn clear() {
    std::process::Command::new("cls").status().unwrap();
}

#[cfg(unix)]
fn clear() {
    std::process::Command::new("clear").status().unwrap();
    //println!("{}[2J", 27 as char);
}
