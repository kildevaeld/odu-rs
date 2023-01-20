use crate::signature::Parameters;
use crate::{arguments::Arguments, Error, Resultable};
use alloc::boxed::Box;
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use odu_value::Value;

pub trait AsyncCallable {
    type Future: Future<Output = Result<Value, Error>>;
    fn parameters(&self) -> Parameters;

    fn call_async(&self, args: Arguments) -> Self::Future;
}

impl<F, U> AsyncCallable for F
where
    F: Fn(Arguments) -> U + Clone,
    U: IntoFuture + 'static,
    U::Output: Resultable,
    <U::Output as Resultable>::Error: Into<Error>,
    <U::Output as Resultable>::Ok: Into<Value>,
{
    type Future = Pin<Box<dyn Future<Output = Result<Value, Error>>>>;

    fn parameters(&self) -> Parameters {
        Parameters::new()
    }

    fn call_async(&self, args: Arguments) -> Self::Future {
        let future = (self)(args);
        let future = async move {
            let ret = future.into_future().await;
            ret.into_result().map(Into::into).map_err(Into::into)
        };

        Box::pin(future)
    }
}
