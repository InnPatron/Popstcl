extern crate popstcl;

use popstcl::vm::internal::*;
use popstcl::parser::*;
use popstcl::vm::user::basic_vm;
use popstcl::vm::internal::Value::*;

#[test]
fn list_eq() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_2 [list [list \"test\" true] 1337];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(vm.inspect_value("list_1").unwrap(), 
               Value::List(vec![
                            Number(1.),
                            Number(2.),
                            Number(3.),
                            Number(4.),
    ]));

    assert_eq!(vm.inspect_value("list_2").unwrap(),
               Value::List(vec![
                           Value::List(vec![
                                       Value::String("test".to_string()),
                                       Bool(true),
                           ]),
                           Number(1337.)
               ]));
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

    assert_eq!(vm.inspect_value("list_1").unwrap(), 
               Value::List(vec![
                            Number(1.),
                            Number(2.),
                            Number(3.),
                            Number(4.),
    ]));

    assert_eq!(vm.inspect_value("list_2").unwrap(),
               Value::List(vec![
                           Value::List(vec![
                                       Value::String("test".to_string()),
                                       Bool(true),
                           ]),
                           Number(1337.)
               ]));
    assert_eq!(vm.inspect_value("var_1").unwrap(),
               Value::Number(1.)
              );
    assert_eq!(vm.inspect_value("var_2").unwrap(),
               Value::Number(1337.)
              );
}

#[test]
fn list_len() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset len [list_len @list_1];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(vm.inspect_value("len").unwrap(),
               Number(4.)
              );
}

#[test]
fn list_append() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_1 [list_append @list_1 5 6 7 8];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(vm.inspect_value("list_1").unwrap(),
               Value::List(vec![
                           Number(1.),
                           Number(2.),
                           Number(3.),
                           Number(4.),
                           Number(5.),
                           Number(6.),
                           Number(7.),
                           Number(8.),
                           ])
               );
}

#[test]
fn list_remove() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset list_1 [list 1 2 3 4];
mset list_1 [list_remove @list_1 0];").unwrap();

    vm.eval_program(&program).unwrap();

    assert_eq!(vm.inspect_value("list_1").unwrap(),
               Value::List(vec![
                           Number(2.),
                           Number(3.),
                           Number(4.),
                           ])
               );
}
