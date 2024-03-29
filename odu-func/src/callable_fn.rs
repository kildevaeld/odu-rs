#[cfg(feature = "async")]
use crate::callable_async::AsyncCallable;
#[cfg(feature = "async")]
use alloc::boxed::Box;
use core::marker::PhantomData;
#[cfg(feature = "async")]
use core::{future::Future, pin::Pin};
use odu_types::StaticTyped;
use odu_value::Value;

use crate::{
    arguments::{Arguments, FromArguments},
    func::Func,
    signature::Signature,
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
    <F::Output as Resultable>::Ok: Into<Value> + StaticTyped,
    <F::Output as Resultable>::Error: Into<Error>,
{
    fn signature(&self) -> Signature {
        Signature::new(
            A::parameters(),
            <<F::Output as Resultable>::Ok as StaticTyped>::typed(),
        )
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

#[cfg(feature = "async")]
impl<F, A> AsyncCallable for CallableFunc<F, A>
where
    for<'a> A: FromArguments<'a> + 'a,
    F: crate::func::Func<A> + Clone + 'static,
    F::Output: Future,
    <F::Output as Future>::Output: Resultable,
    <<F::Output as Future>::Output as Resultable>::Error: Into<Error>,
    <<F::Output as Future>::Output as Resultable>::Ok: Into<Value> + StaticTyped,
{
    type Future<'a> = Pin<Box<dyn Future<Output = Result<Value, Error>> + 'a>>;

    fn signature(&self) -> Signature {
        Signature::new(
            A::parameters(),
            <<<F::Output as Future>::Output as Resultable>::Ok as StaticTyped>::typed(),
        )
    }

    fn call_async<'a>(&'a self, args: Arguments) -> Self::Future<'a> {
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
