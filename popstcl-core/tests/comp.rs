extern crate popstcl_core;

use popstcl_core::vm::internal::*;
use popstcl_core::parser;
use popstcl_core::vm::user::basic_vm;

#[test]
fn comp_cmds() {
    let mut vm = basic_vm();
    let program = parser::parse_program("
mset eq_true [== 12 12];
mset eq_false [== 1337 -1];

mset ineq_true[!= 12 9000];
mset ineq_false [!= 0 0];

mset gr_true [> 1337 -1];
mset gr_false [> 0 10000000];

mset le_true [< 0 9000];
mset le_false [< 1000 -3];

mset greq_true_eq [>= 12 12];
mset greq_true_gr [>= 100000 12];
mset greq_false_le [>= -1 1337];

mset leeq_true_eq [<= 12 12];
mset leeq_true_le [<= 123 100000];
mset leeq_false_gr [<= 1337 -1];"
).unwrap();

    vm.eval_program(&program).unwrap();

    let inspecting = vec![
                            ("eq_true", true.into_value()),
                            ("eq_false", false.into_value()),
                            
                            ("ineq_true", true.into_value()),
                            ("ineq_false", false.into_value()),
                            
                            ("gr_true", true.into_value()),
                            ("gr_false", false.into_value()),
                            
                            ("le_true", true.into_value()),
                            ("le_false", false.into_value()),
                            
                            ("greq_true_eq", true.into_value()),
                            ("greq_true_gr", true.into_value()),
                            ("greq_false_le", false.into_value()),

                            ("leeq_true_eq", true.into_value()),
                            ("leeq_true_le", true.into_value()),
                            ("leeq_false_gr", false.into_value()),
    ];

    for pair in inspecting.iter() {
        let value = vm.inspect_value(pair.0).unwrap();
        let borrow = value.borrow();
        assert_eq!((&pair.0, &*borrow), 
                   (&pair.0, &pair.1));
    }
}
