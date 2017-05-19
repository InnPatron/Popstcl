#![allow(unused_variables)]
use vm::internal::*;
use std::cell::Cell;

const ELSE: &'static str = "else";
const ELIF: &'static str = "elif";

trait IfParser {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr>;
}

enum ParserState {
    IfBool(IfBool),
    IfBody(IfBody),
    Trailing(Trailing),
    ElseBody(ElseBody),
    ElifBool(ElifBool),
    ElifBody(ElifBody),
}

#[derive(Clone, Copy)]
enum TrailingBranch {
    Else,
    Elif,
}

impl ParserState {
    fn new() -> ParserState {
        ParserState::IfBool(IfBool)
    }

    fn check_step(self, stack: &Stack, arg: &CIR) -> Result<Self, ExecErr> {
        match self {
            ParserState::IfBool(ifbool) => {
                ifbool
                    .check(stack, arg)
                    .map(|_| ParserState::IfBody(ifbool.into()))
            }
            ParserState::IfBody(ifbody) => {
                ifbody
                    .check(stack, arg)
                    .map(|_| ParserState::Trailing(ifbody.into()))
            }

            ParserState::Trailing(trail) => {
                trail
                    .check(stack, arg)
                    .map(|_| match trail
                                   .decision
                                   .get()
                                   .expect("Should have been set in check if successful") {
                             TrailingBranch::Else => ParserState::ElseBody(trail.into()),
                             TrailingBranch::Elif => ParserState::ElifBool(trail.into()),
                         })
            }

            ParserState::ElseBody(elsebody) => {
                elsebody
                    .check(stack, arg)
                    .map(|_| ParserState::Trailing(elsebody.into()))
            }

            ParserState::ElifBool(elifbool) => {
                elifbool
                    .check(stack, arg)
                    .map(|_| ParserState::ElifBody(elifbool.into()))
            }

            ParserState::ElifBody(elifbody) => {
                elifbody
                    .check(stack, arg)
                    .map(|_| ParserState::Trailing(elifbody.into()))
            }
        }
    }
}
struct IfBody;
struct IfBool;
struct Trailing {
    decision: Cell<Option<TrailingBranch>>,
}
struct ElseBody;
struct ElifBool;
struct ElifBody;

impl From<IfBool> for IfBody {
    fn from(val: IfBool) -> IfBody {
        IfBody
    }
}

impl From<IfBody> for Trailing {
    fn from(val: IfBody) -> Trailing {
        Trailing { decision: Cell::new(None) }
    }
}

impl From<ElseBody> for Trailing {
    fn from(val: ElseBody) -> Trailing {
        Trailing { decision: Cell::new(None) }
    }
}

impl From<ElifBody> for Trailing {
    fn from(val: ElifBody) -> Trailing {
        Trailing { decision: Cell::new(None) }
    }
}

impl From<Trailing> for ElseBody {
    fn from(val: Trailing) -> ElseBody {
        ElseBody
    }
}

impl From<Trailing> for ElifBool {
    fn from(val: Trailing) -> ElifBool {
        ElifBool
    }
}

impl From<ElifBool> for ElifBody {
    fn from(val: ElifBool) -> ElifBody {
        ElifBody
    }
}

impl IfParser for IfBool {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        if let &CIR::Value(Value::Bool(_)) = arg {
            Ok(())
        } else {
            Err(ExecErr::InvalidArg {
                    expect: "Bool".to_string(),
                    found: arg.clone(),
                })
        }
    }
}

impl IfParser for IfBody {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        if let &CIR::Untouched(ref string) = arg {
            parse_statement_seq(string.trim())
                .map(|_| ())
                .map_err(|err| ExecErr::ParseError(err))
        } else {
            Err(ExecErr::InvalidArg {
                    expect: "If Body".to_string(),
                    found: arg.clone(),
                })
        }
    }
}

impl IfParser for Trailing {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        if let &CIR::Single(ref string) = arg {
            if string == ELSE {
                self.decision.set(Some(TrailingBranch::Else));
            } else if string == ELIF {
                self.decision.set(Some(TrailingBranch::Elif));
            } else {
                return Err(ExecErr::InvalidArg {
                               expect: "Else or Elif".to_string(),
                               found: arg.clone(),
                           });
            }
            Ok(())
        } else {
            Err(ExecErr::InvalidArg {
                    expect: "Else or Elif".to_string(),
                    found: arg.clone(),
                })
        }
    }
}

impl IfParser for ElseBody {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        if let &CIR::Untouched(ref string) = arg {
            parse_statement_seq(string)
                .map(|_| ())
                .map_err(|err| ExecErr::ParseError(err))
        } else {
            Err(ExecErr::InvalidArg {
                    expect: "Else Body".to_string(),
                    found: arg.clone(),
                })
        }
    }
}

impl IfParser for ElifBool {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        if let &CIR::Value(Value::Bool(_)) = arg {
            Ok(())
        } else {
            Err(ExecErr::InvalidArg {
                    expect: "Bool".to_string(),
                    found: arg.clone(),
                })
        }
    }
}

impl IfParser for ElifBody {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        if let &CIR::Untouched(ref string) = arg {
            parse_statement_seq(string)
                .map(|_| ())
                .map_err(|err| ExecErr::ParseError(err))
        } else {
            Err(ExecErr::InvalidArg {
                    expect: "If Body".to_string(),
                    found: arg.clone(),
                })
        }
    }
}
///
/// # Form
/// if bool {}...;
///
/// ... -> else {} ...
/// ... -> elif bool {} ...
///
#[derive(Debug, Clone)]
pub struct If;

impl Cmd for If {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        let mut execute_next = false;
        let mut step = ParserState::new();
        let mut program_seq = None;
        for arg in args.iter() {
            step = step.check_step(stack, arg)?;
            if let &ParserState::ElseBody(_) = &step {
                execute_next = true;
            }

            if let &CIR::Value(Value::Bool(b)) = arg {
                execute_next = b; //read if condition was bool
            }

            if let &CIR::Untouched(ref string) = arg {
                if execute_next {
                    program_seq = Some(parse_statement_seq(string.trim())
                               .expect("Should have been caught by check_step"));
                    break;
                }
            }
        }

        //after running out of arguments: check if in the middle of parsing after an "else", "elif", or elifbool
        use self::ParserState::*;
        match step {
            Trailing(_) => (),      //args ran out after an IfBody, ElseBody, or ElifBody
            IfBool(_) | ElifBool(_) => {
                //looking for condition
                return Err(ExecErr::MissingArg("Looking for bool condition".to_string()));
            },

            IfBody(_) | ElseBody(_) | ElifBody(_) => {
                //looking for body
                return Err(ExecErr::MissingArg("Looking for body".to_string()));
            },
        }

        if let Some(program_seq) = program_seq {
            for entry in program_seq.iter() {
                match eval_some_cmd(stack, &entry.all())? {
                    signal @ ExecSignal::Return(_) => return Ok(signal),
                    signal @ ExecSignal::Continue => return Ok(signal),
                    signal @ ExecSignal::Break => return Ok(signal),
                    ExecSignal::NextInstruction(_) => continue,
                }
            }
        }
        Ok(ExecSignal::NextInstruction(None)) //TODO: ignoring ExecSignal until I find a way to deal with it...
    }
}
