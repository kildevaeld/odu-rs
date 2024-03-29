use crate::signature::{Parameters, Signature};
use crate::{arguments::Arguments, Error, Resultable};
use alloc::boxed::Box;
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use futures_core::future::{BoxFuture, LocalBoxFuture};
use odu_types::StaticTyped;
use odu_value::Value;

pub trait AsyncCallable {
    type Future<'a>: Future<Output = Result<Value, Error>>
    where
        Self: 'a;
    fn signature(&self) -> Signature;

    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a>;
}

pub trait AsyncCallableExt: AsyncCallable {
    fn boxed(self) -> BoxAsyncCallable
    where
        Self: Sized + 'static + Send + Sync,
        for<'a> Self::Future<'a>: Send,
    {
        Box::new(self)
    }

    fn boxed_local(self) -> LocalBoxAsyncCallable
    where
        Self: Sized + 'static + Send + Sync,
    {
        Box::new(self)
    }
}

impl<T> AsyncCallableExt for T where T: AsyncCallable {}

pub type BoxAsyncCallable = Box<dyn internal::BoxAsyncCall + Send + Sync>;

pub type LocalBoxAsyncCallable = Box<dyn internal::BoxLocalAsyncCall + Send + Sync>;

mod internal {
    use futures_core::future::LocalBoxFuture;

    use super::*;

    pub trait BoxAsyncCall {
        fn signature(&self) -> Signature;
        fn call<'a>(&'a self, args: super::Arguments) -> BoxFuture<'a, Result<Value, Error>>;
    }

    impl<T> BoxAsyncCall for T
    where
        T: AsyncCallable,
        for<'a> T::Future<'a>: Send,
    {
        fn signature(&self) -> Signature {
            <T as AsyncCallable>::signature(self)
        }

        fn call<'a>(&'a self, args: super::Arguments) -> BoxFuture<'a, Result<Value, Error>> {
            Box::pin(<T as AsyncCallable>::call_async(self, args))
        }
    }

    pub trait BoxLocalAsyncCall {
        fn signature(&self) -> Signature;
        fn call<'a>(&'a self, args: super::Arguments) -> LocalBoxFuture<'a, Result<Value, Error>>;
    }

    impl<T> BoxLocalAsyncCall for T
    where
        T: AsyncCallable,
    {
        fn signature(&self) -> Signature {
            <T as AsyncCallable>::signature(self)
        }

        fn call<'a>(&'a self, args: super::Arguments) -> LocalBoxFuture<'a, Result<Value, Error>> {
            Box::pin(<T as AsyncCallable>::call_async(self, args))
        }
    }
}

impl AsyncCallable for BoxAsyncCallable {
    type Future<'a> = BoxFuture<'a, Result<Value, Error>>;
    fn signature(&self) -> Signature {
        (**self).signature()
    }
    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
        (**self).call(args)
    }
}

impl AsyncCallable for LocalBoxAsyncCallable {
    type Future<'a> = LocalBoxFuture<'a, Result<Value, Error>>;
    fn signature(&self) -> Signature {
        (**self).signature()
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

    fn signature(&self) -> Signature {
        Signature::new(Parameters::new(), Value::typed())
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
