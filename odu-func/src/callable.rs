#[cfg(feature = "async")]
use crate::AsyncCallable;
use crate::{
    arguments::Arguments,
    error::Error,
    signature::{Parameters, Signature},
};
use alloc::boxed::Box;
#[cfg(feature = "async")]
use core::{marker::PhantomData, pin::Pin};
#[cfg(feature = "async")]
use futures_core::Future;
use odu_types::StaticTyped;

use odu_value::Value;

pub trait Callable {
    fn signature(&self) -> Signature;

    fn call(&self, args: Arguments) -> Result<Value, Error>;
}

impl<F, U, E> Callable for F
where
    F: Fn(Arguments) -> Result<U, E>,
    E: Into<Error>,
    U: Into<Value>,
{
    fn signature(&self) -> Signature {
        Signature::new(Parameters::new(), Value::typed())
    }

    fn call(&self, args: Arguments) -> Result<Value, Error> {
        (self)(args).map(|m| m.into()).map_err(|e| e.into())
    }
}

#[cfg(feature = "async")]
pub trait Executor {
    type Error;
    fn spawn_blocking<F: FnOnce() -> R + 'static + Send, R: Send + 'static>(
        func: F,
    ) -> Pin<Box<dyn Future<Output = Result<R, Self::Error>> + Send>>;
}

#[cfg(feature = "tokio")]
pub struct Tokio;

#[cfg(feature = "tokio")]
impl Executor for Tokio {
    type Error = tokio::task::JoinError;
    fn spawn_blocking<F: FnOnce() -> R + 'static + Send, R: Send + 'static>(
        func: F,
    ) -> Pin<Box<dyn Future<Output = Result<R, Self::Error>> + Send>> {
        Box::pin(tokio::task::spawn_blocking(func))
    }
}

#[cfg(feature = "smol")]
pub struct Smol;

#[cfg(feature = "smol")]
impl Executor for Smol {
    type Error = ();
    fn spawn_blocking<F: FnOnce() -> R + 'static + Send, R: Send + 'static>(
        func: F,
    ) -> Pin<Box<dyn Future<Output = Result<R, Self::Error>> + Send>> {
        Box::pin(async move { Ok(smol::unblock(func).await) })
    }
}

pub trait CallableExt: Callable {
    #[cfg(feature = "async")]
    fn into_async<E>(self) -> IntoAsync<Self, E>
    where
        Self: Sized,
        E: Executor,
    {
        IntoAsync {
            callable: self,
            _executor: PhantomData,
        }
    }

    fn boxed(self) -> Box<dyn Callable>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<C> CallableExt for C where C: Callable {}

#[cfg(feature = "async")]
pub struct IntoAsync<C, E> {
    callable: C,
    _executor: PhantomData<E>,
}

#[cfg(feature = "async")]
impl<C, E> AsyncCallable for IntoAsync<C, E>
where
    C: Callable + Clone + Send + 'static,
    E: Executor + 'static,
    E::Error: core::fmt::Debug + Send + Sync + 'static,
{
    type Future<'a> = Pin<Box<dyn Future<Output = Result<Value, Error>> + Send + 'a>>;
    fn signature(&self) -> Signature {
        self.callable.signature()
    }

    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
        let callable = self.callable.clone();
        Box::pin(async move {
            let ret = E::spawn_blocking(move || callable.call(args))
                .await
                .map_err(|err| Error::Runtime(Box::new(err)))?;

            ret
        })
    }
}
