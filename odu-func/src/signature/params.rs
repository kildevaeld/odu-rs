#[cfg(feature = "validation")]
use crate::arguments::Arguments;
#[cfg(feature = "validation")]
pub use alloc::boxed::Box;
use alloc::{sync::Arc, vec::Vec};
use odu_types::Type;
#[cfg(feature = "validation")]
use odu_validate::{
    validations, ListValidator, ToValidator, Validation, ValidationBox, ValidationError, Validator,
    ValidatorBuilderExt,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Parameters(Option<Arc<Vec<Type>>>);

impl Parameters {
    pub const fn new() -> Parameters {
        Parameters(None)
    }

    pub fn build() -> ParametersBuilder {
        ParametersBuilder {
            params: Vec::default(),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&Type> {
        self.0.as_ref().and_then(|vec| vec.get(idx))
    }

    pub fn iter(&self) -> ParamIter<'_> {
        ParamIter {
            iter: self.0.as_ref().map(|m| m.iter()),
        }
    }

    #[cfg(feature = "validation")]
    pub fn validate(&self, args: &Arguments) -> Result<(), ValidationError> {
        let params = match &self.0 {
            Some(params) => params,
            None => return Ok(()),
        };

        let _arg_types = args.types();

        for (idx, param) in params.iter().enumerate() {
            let arg = match args.get(idx) {
                Some(arg) => arg,
                None => {
                    if param.is_optional() {
                        continue;
                    }

                    panic!("");
                }
            };

            param.validator().validate(arg)?;
            // if !param.is_like(arg) {
            //     panic!("")
            // }
        }
        Ok(())
    }
}

#[cfg(feature = "validation")]
impl ToValidator for Parameters {
    fn validator(&self) -> Validator {
        let mut builder = ListValidator::default();

        if let Some(v) = &self.0 {
            let v = v
                .iter()
                .map(|n| n.validator())
                .map(|v| Box::new(v) as ValidationBox)
                .collect::<Vec<_>>();
            builder = builder.and(validations::tuple(v));
        }

        builder.into()
    }
}

pub struct ParamIter<'a> {
    iter: Option<core::slice::Iter<'a, Type>>,
}

impl<'a> Iterator for ParamIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.iter.as_mut() {
            iter.next()
        } else {
            None
        }
    }
}

pub struct ParametersBuilder {
    params: Vec<Type>,
}

impl ParametersBuilder {
    pub fn with(mut self, param: Type) -> Self {
        self.add(param);
        self
    }

    pub fn add(&mut self, param: Type) -> &mut Self {
        self.params.push(param);
        self
    }

    pub fn build(self) -> Parameters {
        Parameters(Some(Arc::new(self.params)))
    }
}
