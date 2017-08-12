extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn eval() {
    let mut vm = basic_vm();

    vm.eval_str("
set test hello;
eval { mset test2 123; };

assert [== $test hello];
assert [== $test2 123];
").unwrap();
}

#[test]
fn eval_in_mod() {
    let mut vm = basic_vm();

    vm.eval_str("
set test false;
make [std] moddy {
    set test 123;
};

assert [== $test false];
inmod $moddy {assert [== $test 123]; };
").unwrap();
}

#[test]
fn eval_in_place() {
    let mut vm = basic_vm();
    
    vm.eval_str("
proc test {
    eval { mset test_var true; };
    inplace { return 1337; };
};

assert [== [test] 1337];
assert $test_var;
").unwrap();
}
