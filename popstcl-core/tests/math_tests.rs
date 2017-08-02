extern crate popstcl_core;

use popstcl_core::*;

#[test]
fn math_sub() {
    let mut vm = basic_vm();
    vm.eval_str("$mset a [sub 5 -1];").unwrap();
    

    match vm.get("a") {
        Ok(val) => {
            match *val.borrow() {
                Value::Number(ref n) => assert_eq!(6_f64, n.inner()),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn math_add() {
    let mut vm = basic_vm();
    vm.eval_str("$mset a [add 4 3 2 1];").unwrap();
    
    
    match vm.get("a") {
        Ok(val) => {
            match *val.borrow() {
                Value::Number(ref n) => assert_eq!(10_f64, n.inner()),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn math_mul() {
    let mut vm = basic_vm();
    vm.eval_str("$mset a [mul 0.5 3];").unwrap();
    
    
    match vm.get("a") {
        Ok(val) => {
            match *val.borrow() {
                Value::Number(ref n) => assert_eq!(1.5, n.inner()),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn math_div() {
    let mut vm = basic_vm();
    vm.eval_str("$mset a [div 8 2];").unwrap();
    
    
    match vm.get("a") {
        Ok(val) => {
            match *val.borrow() {
                Value::Number(ref n) => assert_eq!(4_f64, n.inner()),
                _   => panic!("'a' is not a number"),
            }
        }

        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn dec_inc() {
    let mut vm = basic_vm();
    vm.eval_str(
"
assert [== [inc 1336] 1337];
assert [== [dec 1338] 1337];

set a 100;
set b [inc $a];
set c [dec[dec $b]];

assert [== $b 101];
assert [== $a 100];
assert [== $c 99];

").unwrap();
}
