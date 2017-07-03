extern crate popstcl_core;

use popstcl_core::vm::internal::*;
use popstcl_core::parser;
use popstcl_core::vm::user::basic_vm;
use popstcl_core::vm::internal::Value::*;

#[test]
fn list_eq() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_2 [list [list \"test\" true] 1337];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(&*vm.inspect_value("list_1").unwrap(), 
               &vec![
                            (1.).into_value(),
                            (2.).into_value(),
                            (3.).into_value(),
                            (4.).into_value(),
               ].into_value()
    );

    assert_eq!(&*vm.inspect_value("list_2").unwrap(),
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
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_2 [list [list \"test\" true] 1337];
mset var_1 [list_get @list_1 0];
mset var_2 [list_get @list_2 1];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(&*vm.inspect_value("list_1").unwrap(), 
               &vec![
                    (1.).into_value(),
                    (2.).into_value(),
                    (3.).into_value(),
                    (4.).into_value(),
               ].into_value()
    );

    assert_eq!(&*vm.inspect_value("list_2").unwrap(),
               &vec![
                           vec![
                                       "test".to_string().into_value(),
                                       true.into_value(),
                           ].into_value(),
                           (1337.).into_value()
               ].into_value());
    assert_eq!(&*vm.inspect_value("var_1").unwrap(),
               &(1.).into_value()
              );
    assert_eq!(&*vm.inspect_value("var_2").unwrap(),
               &(1337.).into_value()
              );
}

#[test]
fn list_len() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset len [list_len @list_1];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(&*vm.inspect_value("len").unwrap(),
               &(4.).into_value()
              );
}

#[test]
fn list_append() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_1 [list_append @list_1 5 6 7 8];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(&*vm.inspect_value("list_1").unwrap(),
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
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_1 [list_remove @list_1 0];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(&*vm.inspect_value("list_1").unwrap(),
               &vec![
                   (2.).into_value(),
                   (3.).into_value(),
                   (4.).into_value(),
               ].into_value()
               );
}
