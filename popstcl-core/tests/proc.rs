extern crate popstcl_core;

use popstcl_core::vm::internal::*;
use popstcl_core::parser;
use popstcl_core::vm::user::basic_vm;

#[test]
fn proc_parse() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number 2} {
    mset test [add ^number1 ^number2];
};").unwrap();
}

#[test]
fn proc_build_obj() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    mset test [add ^number1 ^number2];
};").unwrap();

    vm.eval_program(&program).unwrap();
}

#[test]
fn proc_execute_new_cmd() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    mset test [add ^number1 ^number2];
};").unwrap();

    vm.eval_program(&program).unwrap();

    let mut program = parser::parse_program("
test_proc 21 3;
").unwrap();

    vm.eval_program(&program).unwrap();
    
    match vm.inspect_value("test") {
        Ok(val) => {
            match val {
                Value::Number(ref n) => assert_eq!(24_f64, n.inner()),
                _   => panic!("'test' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn proc_return() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    mset test_1 [add ^number1 ^number2];
    return [mul ^number1 ^number2];
};

mset test_2 [test_proc 3 5];
").unwrap();

    vm.eval_program(&program).unwrap();

    let inspecting = vec![
                            ("test_1", (8_f64).into_value()),
                            ("test_2", (15_f64).into_value()), 
                         ];
    for pair in inspecting.iter() {
        match vm.inspect_value(pair.0) {
            Ok(val) => {
                assert_eq!(pair.1, val);
            },

            Err(_) => panic!("Could not find {}", pair.0),
        }
    }
}

#[test]
fn proc_if_return() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    if false {
        mset test_1 -1;
        return [add ^number1 ^number2];
    } else {
        mset test_1 1337;
        return [mul ^number1 ^number2];
    };
};

mset test_2 [test_proc 3 5];
").unwrap();

    vm.eval_program(&program).unwrap();

    let inspecting = vec![
                            ("test_1", (1337_f64).into_value()),
                            ("test_2", (15_f64).into_value()), 
                         ];
    for pair in inspecting.iter() {
        match vm.inspect_value(pair.0) {
            Ok(val) => {
                assert_eq!(pair.1, val);
            },

            Err(_) => panic!("Could not find {}", pair.0),
        }
    }
}

#[test]
fn proc_empty_return() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    mset test_1 ^number1;
    return;
    mset test_1 ^number2;
};

test_proc 1337 5;
").unwrap();

    vm.eval_program(&program).unwrap();

    let inspecting = vec![
                            ("test_1", (1337_f64).into_value()),
                         ];
    for pair in inspecting.iter() {
        match vm.inspect_value(pair.0) {
            Ok(val) => {
                assert_eq!(pair.1, val);
            },

            Err(_) => panic!("Could not find {}", pair.0),
        }
    }
}

#[test]
#[should_panic]
fn proc_empty_return_in_reduce() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    mset test_1 ^number1;
    return;
    mset test_1 ^number2;
};

mset test_2 [test_proc 1337 5];
").unwrap();

    vm.eval_program(&program).unwrap();

    let inspecting = vec![
                            ("test_1", (1337_f64).into_value()),
                         ];
    for pair in inspecting.iter() {
        match vm.inspect_value(pair.0) {
            Ok(val) => {
                assert_eq!(pair.1, val);
            },

            Err(_) => panic!("Could not find {}", pair.0),
        }
    }
}

#[test]
fn proc_return_in_reduce() {
    let mut vm = basic_vm();
    let mut program = parser::parse_program("
mproc test_proc { number1 number2} {
    add 3 [if true {
                return -21;
            } else {
                return 9000;
            }]
    67;
};

mset test_1 [test_proc 1337 5];
").unwrap();

    vm.eval_program(&program).unwrap();

    let inspecting = vec![
                            ("test_1", (-21_f64).into_value()),
                         ];
    for pair in inspecting.iter() {
        match vm.inspect_value(pair.0) {
            Ok(val) => {
                assert_eq!(pair.1, val);
            },

            Err(_) => panic!("Could not find {}", pair.0),
        }
    }
}
