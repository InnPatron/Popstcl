extern crate popstcl;

use popstcl::vm::internal::*;
use popstcl::parser::*;
use popstcl::vm::user::basic_vm;

#[test]
fn math_sub() {
    let mut vm = basic_vm();
    let program = parser::parse_program("@mset a [sub 5 -1];").unwrap();
    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }
    
    match vm.inspect_value("a") {
        Ok(val) => {
            match val {
                Value::Number(n) => assert_eq!(6_f64, n),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn math_add() {
    let mut vm = basic_vm();
    let program = parser::parse_program("@mset a [add 4 3 2 1];").unwrap();
    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }
    
    match vm.inspect_value("a") {
        Ok(val) => {
            match val {
                Value::Number(n) => assert_eq!(10_f64, n),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn math_mul() {
    let mut vm = basic_vm();
    let program = parser::parse_program("@mset a [mul 0.5 3];").unwrap();
    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }
    
    match vm.inspect_value("a") {
        Ok(val) => {
            match val {
                Value::Number(n) => assert_eq!(1.5, n),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn math_div() {
    let mut vm = basic_vm();
    let program = parser::parse_program("@mset a [div 8 2];").unwrap();
    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }
    
    match vm.inspect_value("a") {
        Ok(val) => {
            match val {
                Value::Number(n) => assert_eq!(4_f64, n),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}
