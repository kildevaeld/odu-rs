use core::fmt;

use odu_types::HasType;

// use crate::ValueType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

// impl PartialEq for Number {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }

// impl Eq for Number {}

// impl PartialOrd for Number {
//     fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
//         todo!()
//     }
// }

// impl Ord for Number {
//     fn cmp(&self, other: &Self) -> core::cmp::Ordering {
//         todo!()
//     }
// }

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Number::U8(i) => write!(f, "{}", i),
            Number::I8(i) => write!(f, "{}", i),
            Number::U16(i) => write!(f, "{}", i),
            Number::I16(i) => write!(f, "{}", i),
            Number::I32(i) => write!(f, "{}", i),
            Number::U32(i) => write!(f, "{}", i),
            Number::I64(i) => write!(f, "{}", i),
            Number::U64(i) => write!(f, "{}", i),
            Number::F32(n) => write!(f, "{}", n),
            Number::F64(n) => write!(f, "{}", n),
        }
    }
}

macro_rules! as_method {
    ($method: ident, $ty: ty) => {
        #[inline]
        pub fn $method(&self) -> $ty {
            self.as_u64() as $ty
        }
    };
}

impl Number {
    // #[inline]
    // pub fn ty(&self) -> ValueType {
    //     match *self {
    //         Number::U8(_) => ValueType::U8,
    //         Number::I8(_) => ValueType::I8,
    //         Number::U16(_) => ValueType::U16,
    //         Number::I16(_) => ValueType::I16,
    //         Number::I32(_) => ValueType::I32,
    //         Number::U32(_) => ValueType::U32,
    //         Number::I64(_) => ValueType::I64,
    //         Number::U64(_) => ValueType::U64,
    //         #[cfg(feature = "ordered_float")]
    //         Number::F32(_) => ValueType::F32,
    //         #[cfg(feature = "ordered_float")]
    //         Number::F64(_) => ValueType::F64,
    //         #[cfg(not(feature = "ordered_float"))]
    //         Number::F32(_) => ValueType::F32,
    //         #[cfg(not(feature = "ordered_float"))]
    //         Number::F64(_) => ValueType::F64,
    //     }
    // }

    // pub fn is(&self, ty: ValueType) -> bool {
    //     self.ty() == ty
    // }

    #[inline]
    pub fn as_u64(&self) -> u64 {
        match *self {
            Number::U8(i) => i as u64,
            Number::I8(i) => i as u64,
            Number::U16(i) => i as u64,
            Number::I16(i) => i as u64,
            Number::I32(i) => i as u64,
            Number::U32(i) => i as u64,
            Number::I64(i) => i as u64,
            Number::U64(i) => i,
            Number::F32(n) => n as u64,
            Number::F64(n) => n as u64,
        }
    }

    as_method!(as_i64, i64);
    as_method!(as_i8, i8);
    as_method!(as_u8, u8);
    as_method!(as_i16, i16);
    as_method!(as_u16, u16);
    as_method!(as_i32, i32);
    as_method!(as_u32, u32);

    #[inline]
    pub fn as_f32(&self) -> f32 {
        match *self {
            Number::U8(i) => i as f32,
            Number::I8(i) => i as f32,
            Number::U16(i) => i as f32,
            Number::I16(i) => i as f32,
            Number::I32(i) => i as f32,
            Number::U32(i) => i as f32,
            Number::I64(i) => i as f32,
            Number::U64(i) => i as f32,
            Number::F32(n) => n,
            Number::F64(n) => n as f32,
        }
    }

    #[inline]
    pub fn as_f64(&self) -> f64 {
        match *self {
            Number::U8(i) => i as f64,
            Number::I8(i) => i as f64,
            Number::U16(i) => i as f64,
            Number::I16(i) => i as f64,
            Number::I32(i) => i as f64,
            Number::U32(i) => i as f64,
            Number::I64(i) => i as f64,
            Number::U64(i) => i as f64,
            Number::F32(n) => n as f64,
            Number::F64(n) => n,
        }
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, Number::F32(_) | Number::F64(_))
    }
}

impl HasType for Number {
    fn typed(&self) -> odu_types::Type {
        use odu_types::{Primitive, Type};
        let ty = match *self {
            Number::U8(_) => Primitive::U8,
            Number::I8(_) => Primitive::I8,
            Number::U16(_) => Primitive::U16,
            Number::I16(_) => Primitive::I16,
            Number::I32(_) => Primitive::I32,
            Number::U32(_) => Primitive::U32,
            Number::I64(_) => Primitive::I64,
            Number::U64(_) => Primitive::U64,
            Number::F32(_) => Primitive::F32,
            Number::F64(_) => Primitive::F64,
        };

        Type::Primitive(ty)
    }
}

macro_rules! expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! arit_impl {
    ($ty: ident, $input: expr, $method: ident, $op: expr) => {
        $ty(v) => Number::$ty(expr!(v $op expr.$method()))
    };
    ($self: ident, $input: expr, $op: tt) => {
        {
            use Number::*;
            match $self {
                U8(v) => U8(expr!(v $op $input.as_u8())),
                I8(v) => I8(expr!(v $op $input.as_i8())),
                U16(v) => U16(expr!(v $op $input.as_u16())),
                I16(v) => I16(expr!(v $op $input.as_i16())),
                U32(v) => U32(expr!(v $op $input.as_u32())),
                I32(v) => I32(expr!(v $op $input.as_i32())),
                U64(v) => U64(expr!(v $op $input.as_u64())),
                I64(v) => I64(expr!(v $op $input.as_i64())),
                F32(v) => F32(expr!(v $op $input.as_f32())),
                F64(v) => F64(expr!(v $op $input.as_f64())),
            }
        }
    };
}

impl<V: Into<Number>> core::ops::Add<V> for Number {
    type Output = Number;
    fn add(self, rhs: V) -> Self::Output {
        arit_impl!(self, rhs.into(), +)
    }
}

impl<V: Into<Number>> core::ops::AddAssign<V> for Number {
    fn add_assign(&mut self, rhs: V) {
        *self = *self + rhs;
    }
}

impl<V: Into<Number>> core::ops::Sub<V> for Number {
    type Output = Number;
    fn sub(self, rhs: V) -> Self::Output {
        arit_impl!(self, rhs.into(), -)
    }
}

impl<V: Into<Number>> core::ops::SubAssign<V> for Number {
    fn sub_assign(&mut self, rhs: V) {
        *self = *self - rhs;
    }
}

impl<V: Into<Number>> core::ops::Mul<V> for Number {
    type Output = Number;
    fn mul(self, rhs: V) -> Self::Output {
        arit_impl!(self, rhs.into(), *)
    }
}

impl<V: Into<Number>> core::ops::MulAssign<V> for Number {
    fn mul_assign(&mut self, rhs: V) {
        *self = *self * rhs;
    }
}

impl<V: Into<Number>> core::ops::Div<V> for Number {
    type Output = Number;
    fn div(self, rhs: V) -> Self::Output {
        arit_impl!(self, rhs.into(), /)
    }
}

impl<V: Into<Number>> core::ops::DivAssign<V> for Number {
    fn div_assign(&mut self, rhs: V) {
        *self = *self / rhs;
    }
}

macro_rules! from_impl {
    ($from: ty, $map: ident) => {
        impl From<$from> for Number {
            fn from(from: $from) -> Number {
                Number::$map(from)
            }
        }
    };
}

from_impl!(u8, U8);
from_impl!(i8, I8);
from_impl!(u16, U16);
from_impl!(i16, I16);
from_impl!(i32, I32);
from_impl!(u32, U32);
from_impl!(i64, I64);
from_impl!(u64, U64);
from_impl!(f32, F32);
from_impl!(f64, F64);
