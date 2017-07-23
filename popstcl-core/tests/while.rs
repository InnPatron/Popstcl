extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn while_test() {
    let mut vm = basic_vm();
    vm.eval_str("
mset index 0;
mset target 100;

while { return [< $index $target]; } {
    mset index [add $index 1];
};").unwrap();

    

    let inspecting = vec![
                            ("index", (100.).into_value()),
    ];

    for pair in inspecting.iter() {
        let value = vm.get(pair.0).unwrap();
        let borrow = value.borrow();
        assert_eq!((&pair.0, &*borrow), 
                   (&pair.0, &pair.1));
    }
}

#[test]
fn while_flow_control() {
    let mut vm = basic_vm();
    vm.eval_str("
mset index 0;
mset target 100;

while { return [< $index $target]; } {
    if [== $index 55] {
        mset index [add $index 2];
        continue;
    };

    if [== $index 57] {
        break;
    };
    mset index [add $index 1];
};").unwrap();

    let inspecting = vec![
                            ("index", (57.).into_value()),
    ];

    for pair in inspecting.iter() {
        let value = vm.get(pair.0).unwrap();
        let borrow = value.borrow();
        assert_eq!((&pair.0, &*borrow), 
                   (&pair.0, &pair.1));
    }   
}
