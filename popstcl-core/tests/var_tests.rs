extern crate popstcl_core;

use popstcl_core::vm::internal::*;
use popstcl_core::parser::*;
use popstcl_core::vm::user::basic_vm;

#[test]
fn sub_in_cmd() {
    let mut vm = basic_vm();
    let program = parser::parse_program("@mset a [@add 1 2];").unwrap();
    vm.eval_program(&program).unwrap();
    
    match vm.inspect_value("a") {
        Ok(val) => {
            match val {
                Value::Number(n) => assert_eq!(3_f64, n),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
#[should_panic]
fn no_ret() {
    let mut vm = basic_vm();
    let program = parser::parse_program("mset a [mlet b 21];").unwrap();
    vm.eval_program(&program).unwrap();
}

#[test]
fn multi_command() {
    let mut vm = basic_vm();
    let program = parser::parse_program("mset a [add [add 3 1] [add [add 1 2] 3]];").unwrap();
    vm.eval_program(&program).unwrap();

    match vm.inspect_value("a") {
        Ok(val) => {
            match val {
                Value::Number(n) => assert_eq!(10_f64, n),
                _               => panic!("'a' not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn let_mset () {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mlet a 21.0 b 1337.0;
mlet a -3.1459;
mlet c @a;
mlet d [add @b -1337 [add 1]];
mset f [mset e 12481632];
mlet g true h false eggs 999;").unwrap();
    
    vm.eval_program(&program).unwrap();

    let inspecting = vec![("a", Value::Number(-3.1459_f64)),
                            ("b", Value::Number(1337_f64)), 
                            ("c", Value::Number(-3.1459)), 
                            ("d", Value::Number(1_f64)), 
                            ("e", Value::Number(12481632_f64)),
                            ("f", Value::Number(12481632_f64)), 
                            ("g", Value::Bool( true)),
                            ("h", Value::Bool(false)),
                            ("eggs", Value::Number(999_f64))
    
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
fn var_sub() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mlet a 21.0 b 1337.0;
mlet a -3.1459;
mlet c @a;
mlet d [add @b -1337 [add 1]];
mset f [mset e 12481632];
mlet g true h false eggs 999;
mset TEST_STRING \"yoyo: @g@f@h b\";").unwrap();
    
    vm.eval_program(&program).unwrap();

    let inspecting = vec![("a", Value::Number(-3.1459_f64)),
                            ("b", Value::Number(1337_f64)), 
                            ("c", Value::Number(-3.1459)), 
                            ("d", Value::Number(1_f64)), 
                            ("e", Value::Number(12481632_f64)),
                            ("f", Value::Number(12481632_f64)), 
                            ("g", Value::Bool( true)),
                            ("h", Value::Bool(false)),
                            ("eggs", Value::Number(999_f64)),
                            ("TEST_STRING", Value::String("yoyo: true12481632false b".to_string())),
    
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
