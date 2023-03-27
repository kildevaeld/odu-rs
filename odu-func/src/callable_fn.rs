use alloc::boxed::Box;
use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use odu_value::Value;

use crate::{
    arguments::{Arguments, FromArguments},
    callable_async::AsyncCallable,
    func::Func,
    signature::Parameters,
    Callable, Error, Resultable,
};

pub struct CallableFunc<F, A> {
    func: F,
    _args: PhantomData<A>,
}

impl<F: Clone, A> Clone for CallableFunc<F, A> {
    fn clone(&self) -> Self {
        CallableFunc {
            func: self.func.clone(),
            _args: PhantomData,
        }
    }
}

impl<F: Copy, A> Copy for CallableFunc<F, A> {}

unsafe impl<F: Send, A> Send for CallableFunc<F, A> {}

unsafe impl<F: Sync, A> Sync for CallableFunc<F, A> {}

impl<F, A> CallableFunc<F, A>
where
    for<'a> A: FromArguments<'a>,
{
    pub fn new(func: F) -> Self
    where
        F: crate::func::Func<A>,
    {
        CallableFunc {
            func,
            _args: PhantomData,
        }
    }
}

impl<F, A> Callable for CallableFunc<F, A>
where
    for<'a> A: FromArguments<'a>,
    F: crate::func::Func<A>,
    F::Output: Resultable,
    <F::Output as Resultable>::Ok: Into<Value>,
    <F::Output as Resultable>::Error: Into<Error>,
{
    fn parameters(&self) -> crate::signature::Parameters {
        A::parameters()
    }
    fn call(&self, args: Arguments) -> Result<Value, Error> {
        let args = A::from_arguments(&args).map_err(|err| err.into())?;

        Ok(self
            .func
            .call(args)
            .into_result()
            .map_err(Into::into)?
            .into())
    }
}

impl<F, A> AsyncCallable for CallableFunc<F, A>
where
    for<'a> A: FromArguments<'a>,
    F: crate::func::Func<A> + Clone + 'static,
    F::Output: Future,
    <F::Output as Future>::Output: Resultable,
    <<F::Output as Future>::Output as Resultable>::Error: Into<Error>,
    <<F::Output as Future>::Output as Resultable>::Ok: Into<Value>,
{
    type Future = Pin<Box<dyn Future<Output = Result<Value, Error>>>>;

    fn parameters(&self) -> Parameters {
        A::parameters()
    }

    fn call_async(&self, args: Arguments) -> Self::Future {
        let func = self.func.clone();
        let future = async move {
            let args = A::from_arguments(&args).map_err(|err| err.into())?;
            let ret = func.call(args).await.into_result().map_err(Into::into)?;
            Ok(ret.into())
        };

        Box::pin(future)
    }
}

pub trait FuncExt<A>: Func<A> {
    fn callable(self) -> CallableFunc<Self, A>
    where
        Self: Sized,
        for<'a> A: FromArguments<'a>,
    {
        CallableFunc::new(self)
    }
}

impl<F, A> FuncExt<A> for F where F: Func<A> {}
