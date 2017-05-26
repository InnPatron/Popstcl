extern crate popstcl;

use popstcl::vm::internal::*;
use popstcl::parser::*;
use popstcl::vm::user::basic_vm;

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
                            ("eq_true", Value::Bool(true)),
                            ("eq_false", Value::Bool(false)),
                            
                            ("ineq_true", Value::Bool(true)),
                            ("ineq_false", Value::Bool(false)),
                            
                            ("gr_true", Value::Bool(true)),
                            ("gr_false", Value::Bool(false)),
                            
                            ("le_true", Value::Bool(true)),
                            ("le_false", Value::Bool(false)),
                            
                            ("greq_true_eq", Value::Bool(true)),
                            ("greq_true_gr", Value::Bool(true)),
                            ("greq_false_le", Value::Bool(false)),

                            ("leeq_true_eq", Value::Bool(true)),
                            ("leeq_true_le", Value::Bool(true)),
                            ("leeq_false_gr", Value::Bool(false)),
    ];

    for pair in inspecting.iter() {
        assert_eq!((&pair.0, &vm.inspect_value(pair.0).expect(&format!("Could not find {}", pair.0))), 
                   (&pair.0, &pair.1));
    }
}