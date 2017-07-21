extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn list_eq() {
    let mut vm = basic_vm();
    vm.eval_str("
mset list_1 [list 1 2 3 4];
mset list_2 [list [list \"test\" true] 1337];").unwrap();

    let value = vm.inspect_value("list_1").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &vec![
                            (1.).into_value(),
                            (2.).into_value(),
                            (3.).into_value(),
                            (4.).into_value(),
               ].into_value()
    );

    let value = vm.inspect_value("list_2").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &vec![
                    vec![
                        "test".to_string().into_value(),
                         true.into_value(),
                    ].into_value(),
                    (1337.).into_value()
               ].into_value()
               );
}

#[test]
fn list_index() {
    let mut vm = basic_vm();
    vm.eval_str("
mset list_1 [list 1 2 3 4];
mset list_2 [list [list \"test\" true] 1337];
mset var_1 [list_get $list_1 0];
mset var_2 [list_get $list_2 1];").unwrap();

    let value = vm.inspect_value("list_1").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow, 
               &vec![
                    (1.).into_value(),
                    (2.).into_value(),
                    (3.).into_value(),
                    (4.).into_value(),
               ].into_value()
    );

    let value = vm.inspect_value("list_2").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &vec![
                           vec![
                                       "test".to_string().into_value(),
                                       true.into_value(),
                           ].into_value(),
                           (1337.).into_value()
               ].into_value());
    let value = vm.inspect_value("var_1").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &(1.).into_value()
              );
    let value = vm.inspect_value("var_2").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &(1337.).into_value()
              );
}

#[test]
fn list_len() {
    let mut vm = basic_vm();
    vm.eval_str("
mset list_1 [list 1 2 3 4];
mset len [list_len $list_1];").unwrap();

    let value = vm.inspect_value("len").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &(4.).into_value()
              );
}

#[test]
fn list_append() {
    let mut vm = basic_vm();
    vm.eval_str("
mset list_1 [list 1 2 3 4];
mset list_1 [list_append $list_1 5 6 7 8];").unwrap();

    let value = vm.inspect_value("list_1").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &vec![
                   (1.).into_value(),
                   (2.).into_value(),
                   (3.).into_value(),
                   (4.).into_value(),
                   (5.).into_value(),
                   (6.).into_value(),
                   (7.).into_value(),
                   (8.).into_value(),
               ].into_value()
               );
}

#[test]
fn list_remove() {
    let mut vm = basic_vm();
    vm.eval_str("
mset list_1 [list 1 2 3 4];
mset list_1 [list_remove $list_1 0];").unwrap();

    let value = vm.inspect_value("list_1").unwrap();
    let borrow = value.borrow();
    assert_eq!(&*borrow,
               &vec![
                   (2.).into_value(),
                   (3.).into_value(),
                   (4.).into_value(),
               ].into_value()
               );
}
