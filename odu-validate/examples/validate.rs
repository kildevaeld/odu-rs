use odu_validate::{
    string, validations, ValidationBox, ValidatorBuilderCommon, ValidatorBuilderExt,
};
use odu_value::Value;

fn main() {
    let v = Box::new(string().required().min(10)) as ValidationBox;

    println!("{}", serde_json::to_string_pretty(&v).unwrap());

    println!(
        "{:?}",
        v.validate(&Value::String(String::from("Hello"))).unwrap()
    );
}
