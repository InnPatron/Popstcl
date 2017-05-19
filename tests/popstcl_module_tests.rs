extern crate popstcl;

use popstcl::vm::internal::*;
use popstcl::parser::*;
use popstcl::vm::user::basic_vm;

#[test]
fn module_from_string() {
    let mut vm = basic_vm();
    let program = parser::parse_program("make other_mod \"
        mset inner_value 1337;
    \";").unwrap();

    for entry in program.code.iter() {
        vm.eval_some_cmd(&entry.all()).unwrap();
    }

    let other_mod = vm.inspect_value("other_mod").expect("Could not find foreign module \'other_mod\'");
    if let Value::Module(other_mod) = other_mod {
        let inner_value = other_mod.get_clone("inner_value").expect("Could not find foreign module value \'inner_value\'");
        if let Value::Number(num) = inner_value {
            assert_eq!(num, 1337.0);
        } else {
            panic!("\'inner_value\' is not Value::Number. Found {}", inner_value);
        }
    } else {
        panic!("\'other_mod\' is not Value::Module. Found {}", other_mod);
    }
}
