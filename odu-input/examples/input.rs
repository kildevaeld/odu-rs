use inquire::Text;
use odu_input::{Input, Ui};
use odu_types::{FromValue, Type, Typed};
use odu_value::Map;

struct Terminal;

impl Ui for Terminal {
    type Error = inquire::InquireError;

    fn text(&self, name: &str, text: &odu_input::Text) -> Result<String, Self::Error> {
        Text::new(name).prompt()
    }

    fn number(
        &self,
        name: &str,
        text: &odu_input::Number,
    ) -> Result<odu_input::Number, Self::Error> {
        todo!()
    }

    fn form(&self, name: &str, form: &odu_input::Form) -> Result<Map, Self::Error> {
        let mut map = Map::default();
        println!("{}", name);
        for field in &form.fields {
            let value = self.input(&field.name, &field.input)?;
            map.insert(&field.name, value);
        }

        Ok(map)
    }
}

#[derive(Debug, Type, FromValue)]
struct Test {
    name: String,
    rapper: String,
    other: Other,
}

#[derive(Debug, Type, FromValue)]
struct Other {
    ost: String,
}

fn main() {
    let ty: Input = Test::typed().into();

    let test: Test = Terminal.input("Test", &ty).unwrap().try_into().unwrap();

    println!("test: {:#?}", test);
}
