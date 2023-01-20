use super::{Arguments, ArgumentsBuilder};
use odu_value::Value;

pub trait ToArguments {
    fn to_arguments(self) -> Arguments;
}

impl ToArguments for () {
    fn to_arguments(self) -> Arguments {
        Arguments::default()
    }
}

impl ToArguments for Arguments {
    fn to_arguments(self) -> Arguments {
        self
    }
}

macro_rules! toargs {
    ($first: ident) => {
        impl<$first: Into<Value>> ToArguments for ($first,)
        {
            fn to_arguments(self) -> Arguments {
                ArgumentsBuilder::default().with(self.0).build()
            }
        }
    };
    ($first: ident $($rest: ident)*) => {
        toargs!($($rest)*);

        impl<$first: Into<Value>, $($rest: Into<Value>),*> ToArguments for ($first, $($rest),*)
        {
            #[allow(non_snake_case)]
            fn to_arguments(self) -> Arguments {
                let mut args = ArgumentsBuilder::default();

                let ($first, $($rest),*) = self;

                args.add($first);

                $(
                    args.add($rest);
                )*

                args.build()

            }
        }
    }
}

toargs!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
