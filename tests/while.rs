extern crate popstcl;

use popstcl::vm::internal::*;
use popstcl::parser::*;
use popstcl::vm::user::basic_vm;

#[test]
fn while_test() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset index 0;
mset target 100;

while { return [< @index @target]; } {
    mset index [add @index 1];
};").unwrap();

    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }

    let inspecting = vec![
                            ("index", Value::Number(100.)),
    ];

    for pair in inspecting.iter() {
        assert_eq!((&pair.0, &vm.inspect_value(pair.0).expect(&format!("Could not find {}", pair.0))), 
                   (&pair.0, &pair.1));
    }
}

#[test]
fn while_flow_control() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset index 0;
mset target 100;

while { return [< @index @target]; } {
    if [== @index 55] {
        mset index [add @index 2];
        continue;
    };

    if [== @index 57] {
        break;
    };
    mset index [add @index 1];
};").unwrap();

    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }

    let inspecting = vec![
                            ("index", Value::Number(57.)),
    ];

    for pair in inspecting.iter() {
        assert_eq!((&pair.0, &vm.inspect_value(pair.0).expect(&format!("Could not find {}", pair.0))), 
                   (&pair.0, &pair.1));
    }   
}
