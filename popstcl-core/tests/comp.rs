extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn comp_cmds() {
    let mut vm = basic_vm();
    vm.eval_str("
 assert [== 12 12];
 assert [inv [== 1337 -1]];

 assert [!= 12 9000];
 assert [inv  [!= 0 0]];

 assert [> 1337 -1];
 assert [inv [> 0 10000000]];

 assert [< 0 9000];
 assert [inv [< 1000 -3]];

 assert [>= 12 12];
 assert [>= 100000 12];
 assert [inv [>= -1 1337]];

 assert [<= 12 12];
 assert [<= 123 100000];
 assert [inv [<= 1337 -1]];"
).unwrap();
}

#[test]
fn ref_comp() {
    let mut vm = basic_vm();
    vm.eval_str("mset obj_1 [object f1 hello f2 world];
set field false;
mset obj_2 $obj_1;
mmut obj_3 $obj_1;
assert [=== $obj_1 $obj_2];
assert [!=== $obj_1 $obj_3];
assert [== $obj_1 $obj_3];
assert [== $obj_2 $obj_3];
assert [!= $obj_1 $field];
assert [!=== $obj_1 $field];").unwrap();
}

#[test]
fn invert() {
    let mut vm = basic_vm();
    vm.eval_str(
"assert [inv false];").unwrap();
}
