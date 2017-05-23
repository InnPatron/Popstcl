# The Popstcl Language

## What is It?

POPSTCL (stylized as Popstcl because it feels too shouty) stands for *Programmer's Oddly Planned Simple Tool Command Language*. It is a [Tcl](https://www.tcl.tk/) variant and a **hobby language** meant to practice [Rust](https://www.rust-lang.org/en-US/) as well as write my first usable interpreter.

I did **not**:
* Implement all the standard Tcl commands
* Read any Tcl specification
* Fully follow any [Tcl tutorial](https://www.tcl.tk/doc/)

**Popstcl is the product of my first impressions of the Tcl and may very well be inaccurate (and incomplete).**

Explicit **non-goals**:
* Strive to be used in a production environment (That being said, I do plan on making a text adventure game as a rite of passage)
* Full compatiblity with vanilla Tcl

## Differences to Tcl

### Command Invocation
This is valid Popstcl code:
~~~~
$cmd_name arg1 arg2;
~~~~
where "cmd" is a valid binding pointing to a command value.

### Python/Javascript-esque Objects
Objects are dictionaries.

TODO: Example

TODO: Example of value access

### Modules
Each Popstcl script runs in its own environment with predefined commands and values. That environment may be specified by the command invoker.

Modules may be stored as values and are objects internally.

TODO: Example of loading a module

TODO: Example of value access

### Scoping
There are three levels of scoping each with their own substitution sigil:
1. Module-level 
2. Local-level
3. Procedure-level

~~~
print $local_variable;
print @module_variable;
print ^proc_arg;
~~~

## Extensibility

Commands can be implemented through the popstcl::vm::cmd::Cmd trait and passed as Box<Cmd>. Eventually, I want to be able to interact with C data and commands.

## Cool Things

### Moving and non-moving closures are easy to implement.
TODO: Add example

...And more!
