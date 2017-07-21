extern crate popstcl_core;
use popstcl_core::*;

#[test]
fn program_return() {
    let mut vm = basic_vm();
    match vm.eval_str("mset a 5; return \"hello\"; mmut a 10;").unwrap() {
        Some(rcv) => {
            if let Value::String(ref s) = *rcv.borrow() {
                assert_eq!(&***s, "hello");
            } else {
                panic!("Expected popstcl program to return \"hello\"");
            }
        },

        None => panic!("expected value from program"),
    }

    assert_eq!((5.).into_value(), vm.get_value("a").unwrap());
}

#[test]
fn program_handle_return() {
    let mut vm = basic_vm();
    let handle = match vm.eval_str("mset a 5; return $a;").unwrap() {
        Some(rcv) => {
            rcv
        },

        None => panic!("expected value from program"),
    };

    vm.eval_str("mmut a -1;").unwrap();

    assert_eq!((-1.).into_value(), vm.get_value("a").unwrap());
    assert_eq!(*handle.borrow(), vm.get_value("a").unwrap());
}
