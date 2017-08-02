extern crate popstcl_core;
use popstcl_core::*;

#[test]
fn assert_pass() {
    let mut vm = basic_vm();
    vm.eval_str(
"assert true;
assert [== hello hello];
assert [== false false];
assert [!= -1 10000000];
assert [== [list 1 2 3] [list 1 2 3]];").unwrap();
}

#[test]
#[should_panic]
fn assert_fail() {
    let mut vm = basic_vm();
    vm.eval_str(
"
assert false;
").unwrap();
}

#[test]
#[should_panic]
fn assert_fail_eq() {
    let mut vm = basic_vm();
    vm.eval_str(
"assert [!= 1000 1000];").unwrap();
}
