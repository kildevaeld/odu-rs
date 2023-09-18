#[cfg(feature = "async")]
use crate::AsyncCallable;
use crate::{arguments::Arguments, error::Error, signature::Parameters};
use alloc::boxed::Box;
#[cfg(feature = "async")]
use core::{marker::PhantomData, pin::Pin};
#[cfg(feature = "async")]
use futures_core::Future;

use odu_value::Value;

pub trait Callable {
    fn parameters(&self) -> Parameters;

    fn call(&self, args: Arguments) -> Result<Value, Error>;
}

impl<F, U, E> Callable for F
where
    F: Fn(Arguments) -> Result<U, E>,
    E: Into<Error>,
    U: Into<Value>,
{
    fn parameters(&self) -> Parameters {
        Parameters::new()
    }

    fn call(&self, args: Arguments) -> Result<Value, Error> {
        (self)(args).map(|m| m.into()).map_err(|e| e.into())
    }
}

#[cfg(feature = "async")]
pub trait Executor {
    fn spawn_blocking<F, R>(func: F) -> Pin<Box<dyn Future<Output = R> + Send>>;
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
{
    type Future<'a> = Pin<Box<dyn Future<Output = Result<Value, Error>> + Send + 'a>>;
    fn parameters(&self) -> Parameters {
        self.callable.parameters()
    }

    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
        let callable = self.callable.clone();
        E::spawn_blocking(move || callable.call(args))
    }
}
