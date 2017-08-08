extern crate popstcl_core;
#[macro_use]
extern crate popstcl_derive;

use popstcl_core::*;

#[derive(IntoValue)]
pub struct UnitStruct;

#[derive(IntoValue)]
pub struct Struct {
    fl: f64,
    b: bool,
}

#[derive(IntoValue)]
pub struct TupleStruct(String, String, f64);

#[derive(IntoValue)]
pub struct EmptyTuple();

#[derive(IntoValue)]
pub struct EmptyStruct { }

#[derive(IntoValue)]
pub struct NestedDerive {
    unit: UnitStruct,
    reg_struct: Struct,
    tuple: TupleStruct,
    empty: EmptyStruct,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_retrieval() {
        {
            let value = TupleStruct("hello".to_string(), "world".to_string(), 100.0).into_value();
            let obj = value.try_into_object().unwrap();
            assert_eq!("hello", 
                       &**obj.get("0")
                            .unwrap()
                            .inner_clone()
                            .try_into_string()
                            .unwrap());
            assert_eq!("world",
                       &**obj.get("1")
                            .unwrap()
                            .inner_clone()
                            .try_into_string()
                            .unwrap());
            assert_eq!(100.0,
                       *obj.get("2")
                           .unwrap()
                           .inner_clone()
                           .try_into_number()
                           .unwrap());
        }

        {
            let value = Struct { fl: 100.0, b: false }.into_value();
            let obj = value.try_into_object().unwrap();
            assert_eq!(100.0,
                       *obj.get("fl")
                            .unwrap()
                            .inner_clone()
                            .try_into_number()
                            .unwrap());
            assert_eq!(false,
                       *obj.get("b")
                            .unwrap()
                            .inner_clone()
                            .try_into_bool()
                            .unwrap());
        }
    }
}
