extern crate popstcl_core;

use popstcl_core::vm::internal::*;
use popstcl_core::parser;
use popstcl_core::vm::user::basic_vm;

#[test]
fn if_parsing() {
    let word = parser::parse_program(
"
if true {
mset a true;
mset b true;
} else if true {

} else {

};
").unwrap();
    println!("Success");
}

#[test]
fn if_executing_true() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset foo true;
mset output 0;
if $foo {
    mset output 1;
};
    ").unwrap();
    vm.eval_program(&program).unwrap();
    let inspecting = vec!["output"];
    for element in inspecting.iter() {
        match vm.inspect_value(element) {
            Ok(val) => {
                if let Value::Number(ref n) = *val.borrow() {
                    assert_eq!(1_f64, n.inner());
                } else {
                    panic!("output not a number");
                }
            }

            Err(_) => panic!("Could not find {}", element),
        }
    }
}

#[test]
fn if_executing_false() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset foo false;
mset output 1337;
if $foo {
    mset output 1;
};
    ").unwrap();
    vm.eval_program(&program).unwrap();
    let inspecting = vec!["output"];
    for element in inspecting.iter() {
        match vm.inspect_value(element) {
            Ok(val) => {
                if let Value::Number(ref n) = *val.borrow() {
                    assert_eq!(1337_f64, n.inner());
                } else {
                    panic!("output not a number");
                }
            }

            Err(_) => panic!("Could not find {}", element),
        }
    }
}

#[test]
fn if_executing_else() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset foo false;
mset bar false;
mset baz false;

mset output 1337;
if $foo {
    mset output 1;
} elif $bar {
    mset output 2;
} elif $baz {
    mset output 3;
} else {
    mset output 9000;
};
    ").unwrap();
    vm.eval_program(&program).unwrap();
    let inspecting = vec!["output"];
    for element in inspecting.iter() {
        match vm.inspect_value(element) {
            Ok(val) => {
                if let Value::Number(ref n) = *val.borrow() {
                    assert_eq!(9000_f64, n.inner());
                } else {
                    panic!("output not a number");
                }
            }

            Err(_) => panic!("Could not find {}", element),
        }
    }
}

#[test]
fn if_executing_elif() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset foo false;
mset bar true;
mset baz true;

mset output 1337;
if $foo {
    mset output 1;
} elif $bar {
    mset output 2;
} elif $baz {
    mset output 3;
} else {
    mset output 9000;
};
    ").unwrap();
    vm.eval_program(&program).unwrap();
    let inspecting = vec!["output"];
    for element in inspecting.iter() {
        match vm.inspect_value(element) {
            Ok(val) => {
                if let Value::Number(ref n) = *val.borrow() {
                    assert_eq!(2_f64, n.inner());
                } else {
                    panic!("output not a number");
                }
            }

            Err(_) => panic!("Could not find {}", element),
        }
    }
}
