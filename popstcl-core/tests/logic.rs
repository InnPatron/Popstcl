extern crate popstcl_core;
use popstcl_core::*;

#[test]
fn and_or_not_pass() {
    let mut vm = basic_vm();
    vm.eval_str(
"assert [&& true true];
assert [!![&& false false]];
assert [|| false true];
assert [|| true false];
assert [|| true true];
").unwrap();
}

#[test]
#[should_panic]
fn and_fail() {
    let mut vm = basic_vm();
    vm.eval_str(
"
assert [&& false false];
").unwrap();
}

#[test]
#[should_panic]
fn or_fail() {
    let mut vm = basic_vm();
    vm.eval_str(
"
assert [|| false false];
").unwrap();
}

#[test]
#[should_panic]
fn inv_fail() {
    let mut vm = basic_vm();
    vm.eval_str(
"
assert [! [&& true true]];
").unwrap();
}
