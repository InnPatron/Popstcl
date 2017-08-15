extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn module_var_sub() {
    let mut vm = basic_vm();
    vm.eval_str("
    set test 12345;
    assert [== \"12345\" \"$test\"];
").unwrap();
}

#[test]
fn local_var_sub() {
    let mut vm = basic_vm();
    vm.eval_str("
mset test 2468;
proc local_var_sub {
    lset test -1;
    assert [== \"-1\" \"#test\"];
};
local_var_sub;
").unwrap();
}

#[test]
fn arg_var_sub() {
    let mut vm = basic_vm();
    vm.eval_str("
proc arg_var_sub { arg1 arg2 } {
    assert [== \"hello\" @arg1];
    assert [== \"world\" @arg2];
};

arg_var_sub hello world;
").unwrap();
}

#[test]
fn object_field_sub() {
    let mut vm = basic_vm();
    vm.eval_str("
set test [object name { object name }];
set test2 [object obj $test];
assert [== \" object name \" \"$test2.obj.name\"];
").unwrap();
}
