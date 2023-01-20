use core::{marker::PhantomData, pin::Pin};

use crate::{arguments::Arguments, error::Error, signature::Parameters, AsyncCallable};

use alloc::boxed::Box;
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

pub trait Executor {
    fn spawn_blocking<F, R>(func: F) -> Pin<Box<dyn Future<Output = R> + Send>>;
}

pub trait CallableExt: Callable {
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
}

impl<C> CallableExt for C where C: Callable {}

pub struct IntoAsync<C, E> {
    callable: C,
    _executor: PhantomData<E>,
}

impl<C, E> AsyncCallable for IntoAsync<C, E>
where
    C: Callable + Clone + Send,
    E: Executor,
{
    type Future = Pin<Box<dyn Future<Output = Result<Value, Error>> + Send>>;
    fn parameters(&self) -> Parameters {
        self.callable.parameters()
    }

    fn call_async(&self, args: Arguments) -> Self::Future {
        let callable = self.callable.clone();
        E::spawn_blocking(move || callable.call(args))
    }
}
