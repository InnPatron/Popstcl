extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn module_from_string() {
    let mut vm = basic_vm();
    vm.eval_str(
        "make other_mod \"
        mset inner_value 1337;
    \";").unwrap();

    let other_mod = vm.get("other_mod").expect("Could not find foreign module \'other_mod\'");
    let borrow = other_mod.borrow();
    if let Value::Module(ref other_mod) = *borrow {
        let inner_value = other_mod.get("inner_value").expect("Could not find foreign module value \'inner_value\'");
        let borrow = inner_value.borrow();
        if let Value::Number(ref num) = *borrow {
            assert_eq!(num.inner(), 1337.0);
        } else {
            panic!("\'inner_value\' is not Value::Number. Found {}", inner_value);
        }
    } else {
        panic!("\'other_mod\' is not Value::Module. Found {}", other_mod);
    }
}
