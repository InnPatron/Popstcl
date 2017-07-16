# Starting Out
See popstcl/tests or the Tcl/Tk website for more information.
Interpreter found [here](https://gitlab.com/Random_Civvy/popstcl-i)

## Comments
**NOT YET IMPLEMENTED**

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
print @module_level;
print $local_level;
print ^procedure_argument;
~~~

## Command Substitutions
~~~
mset binding_name [add 1 2 3 4 5 6];
~~~

## Command Cheatsheet
**ONLY VALID WITH DEFAULT ENVIRONMENT.** ALL command names are configureable.

### Creating Variables

TODO: explain difference between set and let
~~~
mset module_level_name @value;
mlet module_level_name @value;
~~~
'm' stands for "module-level."

### Comparators
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
sub 10 10 2; 	// Returns -2
div 100 2 5; 	// Returns 10
mul 3 6 2; 		// Returns 36
~~~

### Procedure

~~~
proc proc_name { arg_1 arg_2 } {
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
	//body
};
~~~
