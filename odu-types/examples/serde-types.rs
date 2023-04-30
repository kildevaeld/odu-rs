use odu_types::{ComplexType, Type};

#[derive(odu_types::Type)]
struct TestType {
    name: String,
    age: u32,
}

const INPUT: &str = r#"
{
    "name": "TestType",
    "fields": [
      {
        "name": "age",
        "kind": "U32"
      },
      {
        "name": "name",
        "kind": "String"
      }
    ]
  }
  
   
"#;

fn main() {
    let ty = odu_types::type_id::<TestType>();

    let ty = Type::Complex(ty);

    let o: ComplexType = serde_json::from_str(INPUT).expect("msg");

    // println!("{:#?}", odu_types::type_info(ty));

    //let o = serde_json::to_string_pretty(&ty).expect("serde");

    println!("{:?}", o);
}
