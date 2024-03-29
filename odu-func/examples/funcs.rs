use odu_func::{
    arguments::{Arguments, ArgumentsBuilder, ToArguments},
    AsyncCallable, AsyncCallableExt, Callable, CallableFunc, Error, FromValue, FuncExt, IntoValue,
    Type,
};
use odu_validate::ToValidator;

#[derive(IntoValue, FromValue, Type)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn test(self, string: &str) {}
}

#[derive(IntoValue, FromValue, Type)]
struct Test {
    person: Person,
}

fn test(test: Test) -> Result<String, Error> {
    Ok(String::from("Test func"))
}

fn main() -> Result<(), Error> {
    let action = CallableFunc::new(|person: Person| async move {
        Result::<_, Error>::Ok(format!("Hello, {}", person.name))
    })
    .boxed_local();

    let callable = test.callable();

    // let ret = callable.call(Arguments::default())?;

    let ret = callable.call(
        (Test {
            person: Person {
                name: "Test".to_string(),
                age: 1,
            },
        },)
            .to_arguments(),
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&callable.signature().params().validator()).unwrap()
    );

    let args = ArgumentsBuilder::default()
        .with(Person {
            name: "World".into(),
            age: 6,
        })
        .build();

    let result = futures_executor::block_on(action.call_async(args))?;

    //let result = action.call(args)?;

    println!("{:?}", callable.signature());

    Ok(())
}
