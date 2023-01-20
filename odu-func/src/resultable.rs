pub trait Resultable {
    type Ok;
    type Error;
    fn into_result(self) -> Result<Self::Ok, Self::Error>;
}

impl<T, E> Resultable for Result<T, E> {
    type Ok = T;
    type Error = E;

    fn into_result(self) -> Result<T, E> {
        self
    }
}
