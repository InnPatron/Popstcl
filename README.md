# The Popstcl Language
This repository holds the core language library and commands for embedding.

## What is It?

POPSTCL (stylized as Popstcl because to be less shouty) stands for *Programmer's Oddly Planned Simple Tool Command Language*. It is a [Tcl](https://www.tcl.tk/) variant and a **hobby language** meant to practice [Rust](https://www.rust-lang.org/en-US/) and to use embedded in other applications.

I did **not**:
* Implement all the standard Tcl commands
* Read any Tcl specification
* Fully follow any [Tcl tutorial](https://www.tcl.tk/doc/)

**Popstcl is the product of my first impressions of the Tcl and may very well be inaccurate (and incomplete).**

Explicit **non-goals**:
* Strive to be used in a production environment (That being said, I do plan on making a text adventure game as a rite of passage)
* Full compatiblity with vanilla Tcl

## The Interpreter
Currently, [popstcl-i](https://gitlab.com/Random_Civvy/popstcl-i) is an extremely barebones REPL. 

### Other Projects
* A choose your own adventure engine / game using Popstcl as a scripting language: [CYOA] (https://gitlab.com/Random_Civvy/cyoa)

## Differences to Tcl (Incomplete)

### Command Invocation
This is valid Popstcl code:
~~~~
$cmd_name arg1 arg2;
"other cmd name" arg1 arg2;
~~~~
where "cmd" is a valid binding pointing to a command value.

### Python/Javascript-esque Objects
Objects are dictionaries. Inheritance is achieved via prototyping

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
print $module_variable;
print #local_variable;
print @proc_arg;
~~~

## Extensibility

Commands can be implemented through the popstcl::vm::cmd::Cmd trait and passed as Box<Cmd>. Eventually, I want to be able to interact with C data.

## Cool Things

### Moving Closures
TODO: Add example

...And more!

## Versioning
Currently, releases follow semver for popstcl-core.

## License
Released under the [MIT License](https://opensource.org/licenses/MIT) (See LICENSE-MIT). Good luck using this for any commercial projects :|
