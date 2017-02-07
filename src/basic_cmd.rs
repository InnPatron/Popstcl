use common_obj::*;
use ast::*;
use err::*;

pub struct Set;

impl Command for Set {

    fn get_type(&self) -> BindType {
        BindType::Void
    }

    fn execute(&self, context: &mut Context, args: &Vec<Word>) -> Result<(), ExecErr> {
       
        Ok(())
    }
}
