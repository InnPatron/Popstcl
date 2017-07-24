# Starting Out
See popstcl/tests or the Tcl/Tk website for more information.
Interpreter found [here](https://gitlab.com/Random_Civvy/popstcl-i)

## General Structure
Execution revolves around statements. Statements end with semicolons. Words are separated by select denominators or whitespace.

"!bang" is a single word.
"one two" are separate words.
"abcd; efgh;" are two statements.

The first word in a statement must be either:
1. The name of a command, queried in this order of namespaces: argument -> any_nested_local -> module
2. A Popstcl string which resolves to the name of a command (using the above order)
3. A command or variable substitution which reduces to a command

**ALL VALUES INSERTED INTO THE VM ARE REFERENCE COUNTED.** Bindings are reprsented as ref-counted pointers in a dictionary.

## Embedding
Popstcl code is executed in the Vm.
You may precompile a string or feed it directly to the VM.

In Rust:
~~~
extern crate popstcl_core;
use popstcl_core::*;

let mut vm = basic_vm();		//Vm with only std commands loaded
let program = parse_program("bla"); 	//returns a Program struct
vm.eval_program(&program);
vm.eval_strign("bla");
~~~

See popstcl-core/src/vm/mod.rs for VM definition

# Language

## Comments
~~~
command stuff;
// This is a comment
~~~

## Data Types (Incomplete)
* Number (floating point, 64bits)
* Bool
* String (UTF-8)
* List
* Object (Dictionary)
* Module (Dictionary)
* Cmd

## Variable Substitutions
~~~
print $module_level;
print #local_level;
print @procedure_argument;

//Variable substitution can follow paths through an object
print @object_root.child.child.child;	//Valid if object_root has a field named child, with another field named child, with another field named child
~~~

## Command Substitutions
~~~
mset binding_name [add 1 2 3 4 5 6];
~~~

## Command Cheatsheet
**ONLY VALID WITH STD ENVIRONMENT.** ALL command names are configureable.

### Variables

~~~
set module_level_name 4;	//Inserts or overrides binding with new ref-counted pointer pointing at the location of the "4" value
let a 1 b 2 c 3;		//Inserts or overrides multiple bindings
mut a 3;			//Dereferences "a" variable and sets it to 3. DOES NOT CHANGE ANY POINTERS

//'m' stands for "module-level." May or may not be present
//'f' stands for "field-level." Manipulates the fields of an object
//Local commands are prefixed by an 'l'
~~~

### Comparators
**NO AUTOMATIC COERCIONS**
Use math symbols and return bools.
Example:
~~~
mset truth [== 1 1];
mset another_truth [<= 0 9000];
mset false [> -5 100];
mset another_false[!= 2 2];
~~~

### Math
~~~
add -100; 		// Returns -100
add 15 3; 		// Returns 18
sub 10 10 2; 		// Returns -2
div 100 2 5; 		// Returns 10
mul 3 6 2; 		// Returns 36
~~~

### Procedure

~~~
//proc is merely a **command** which creates a command
proc proc_name { arg_1 arg_2 } {
	//procedure_body
};

proc no_args { 
	//procedure_body
};
~~~

### Flow control

#### If
~~~ 
if @bool_value {
	//body
} elif @bool_value {
	//body
} else {
	//body
};
~~~

#### While
~~~
while "String that eventually returns a bool" {
	if true {
		continue;
	} else if false {
		break;
	} else {
		return;
	}
};
~~~

### Objects
Popstcl represents objects as dictionaries.

~~~
mset a [object]; 			//create empty object
mset b [object f1 0 f2 "hello_world"]; 	//create an object with two fields
fmut $b f1 false; 			//mutates field f1 on object b to be false
~~~

### Miscillaneous
~~~
print hello; 		//hello
print false; 		//false
print [object]; 	//Object[]
eprint "HALP ME";	//print to StdErr

assert true;
assert false;		//Popstcl program aborts and returns error message
~~~
