#![allow(unused_variables)]
use vm::internal::*;
use std::cell::Cell;
use parser::parse_program;

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
        cir_extract!(arg => Bool, "If Condition").map(|_| ())
    }
}

impl IfParser for IfBody {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        parse_program(cir_extract!(arg => String, "If Body")?)
            .map(|_| ())
            .map_err(|e| ExecErr::ParseError(e))
    }
}

impl IfParser for Trailing {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        let string = cir_extract!(arg => String)?;
        
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
    }
}

impl IfParser for ElseBody {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        parse_program(cir_extract!(arg => String, "Else Body")?)
            .map(|_| ())
            .map_err(|err| ExecErr::ParseError(err))
    }
}

impl IfParser for ElifBool {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        cir_extract!(arg => Bool, "Elif Condition")
            .map(|_| ())
    }
}

impl IfParser for ElifBody {
    fn check(&self, stack: &Stack, arg: &CIR) -> Result<(), ExecErr> {
        parse_program(cir_extract!(arg => String, "Elif Body")?)
            .map(|_| ())
            .map_err(|err| ExecErr::ParseError(err))
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
                continue;
            }

            if let Value::Bool(b) = arg.value {
                execute_next = b; //read if condition was bool
            }

            if let Value::String(ref string) = arg.value {
                if execute_next {
                    program_seq = Some(parse_program(string.trim())
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

            IfBody(_) => return Err(ExecErr::MissingArg("If Body".to_string())),

            ElseBody(_) => return Err(ExecErr::MissingArg("Else Body".to_string())),

            ElifBody(_) => return Err(ExecErr::MissingArg("Elif Body".to_string())),
        }

        if let Some(program_seq) = program_seq {
            for stmt in program_seq.iter() {
                match eval_stmt(stack, stmt)? {
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
