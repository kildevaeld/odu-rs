use crate::signature::Parameters;
use crate::{arguments::Arguments, Error, Resultable};
use alloc::boxed::Box;
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use futures_core::future::{BoxFuture, LocalBoxFuture};
use odu_value::Value;

pub trait AsyncCallable {
    type Future<'a>: Future<Output = Result<Value, Error>>
    where
        Self: 'a;
    fn parameters(&self) -> Parameters;

    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a>;
}

pub trait AsyncCallableExt: AsyncCallable {
    fn boxed(self) -> BoxAsyncCallable
    where
        Self: Sized + 'static,
        for<'a> Self::Future<'a>: Send,
    {
        Box::new(self)
    }
}

pub type BoxAsyncCallable = Box<dyn internal::BoxAsyncCall>;

pub type LocalBoxAsyncCallable = Box<dyn internal::BoxLocalAsyncCall>;

mod internal {
    use futures_core::future::LocalBoxFuture;

    use super::*;

    pub trait BoxAsyncCall {
        fn parameters(&self) -> Parameters;
        fn call<'a>(&'a self, args: super::Arguments) -> BoxFuture<'a, Result<Value, Error>>;
    }

    impl<T> BoxAsyncCall for T
    where
        T: AsyncCallable,
        for<'a> T::Future<'a>: Send,
    {
        fn parameters(&self) -> Parameters {
            <T as AsyncCallable>::parameters(self)
        }

        fn call<'a>(&'a self, args: super::Arguments) -> BoxFuture<'a, Result<Value, Error>> {
            Box::pin(<T as AsyncCallable>::call_async(self, args))
        }
    }

    pub trait BoxLocalAsyncCall {
        fn parameters(&self) -> Parameters;
        fn call<'a>(&'a self, args: super::Arguments) -> LocalBoxFuture<'a, Result<Value, Error>>;
    }

    impl<T> BoxLocalAsyncCall for T
    where
        T: AsyncCallable,
    {
        fn parameters(&self) -> Parameters {
            <T as AsyncCallable>::parameters(self)
        }

        fn call<'a>(&'a self, args: super::Arguments) -> LocalBoxFuture<'a, Result<Value, Error>> {
            Box::pin(<T as AsyncCallable>::call_async(self, args))
        }
    }
}

impl AsyncCallable for BoxAsyncCallable {
    type Future<'a> = BoxFuture<'a, Result<Value, Error>>;
    fn parameters(&self) -> Parameters {
        (**self).parameters()
    }
    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
        (**self).call(args)
    }
}

impl AsyncCallable for LocalBoxAsyncCallable {
    type Future<'a> = LocalBoxFuture<'a, Result<Value, Error>>;
    fn parameters(&self) -> Parameters {
        (**self).parameters()
    }
    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
        (**self).call(args)
    }
}

impl<F, U> AsyncCallable for F
where
    F: Fn(Arguments) -> U + Clone,
    for<'a> F: 'a,
    for<'a> U: IntoFuture + 'a,
    U::Output: Resultable,
    <U::Output as Resultable>::Error: Into<Error>,
    <U::Output as Resultable>::Ok: Into<Value>,
{
    type Future<'a> = Pin<Box<dyn Future<Output = Result<Value, Error>> + 'a>>;

    fn parameters(&self) -> Parameters {
        Parameters::new()
    }

    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
        let future = (self)(args);
        let future = async move {
            let ret = future.into_future().await;
            ret.into_result().map(Into::into).map_err(Into::into)
        };

        Box::pin(future)
    }
}
