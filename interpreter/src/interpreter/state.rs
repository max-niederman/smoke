use super::{Error, Result};
use crate::parser::ast::Literal;
use crate::{extract, extract_variant_method};
use std::cell::RefCell;
use std::cmp;
use std::fmt;
use std::ops;
use std::rc::Rc;

/// A runtime Smoke value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(isize),
    Float(f64),
    Str(String),
}

impl Value {
    pub fn wrapped(self) -> ValueWrap {
        ValueWrap::wrapping(self)
    }

    #[allow(dead_code)]
    pub const fn as_nil(&self) -> Option<()> {
        match self {
            Self::Nil => Some(()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn into_nil(self) -> Option<()> {
        self.as_nil()
    }

    extract_variant_method!(as_bool(&self) { Self::Bool as (a): (&bool) });
    extract_variant_method!(as_int(&self) { Self::Integer as (a): (&isize) });
    extract_variant_method!(as_float(&self) { Self::Float as (a): (&f64) });
    extract_variant_method!(as_str(&self) { Self::Str as (a): (&str) });

    extract_variant_method!(into_bool(self) { Self::Bool as (a): (bool) });
    extract_variant_method!(into_int(self) { Self::Integer as (a): (isize) });
    extract_variant_method!(into_float(self) { Self::Float as (a): (f64) });
    extract_variant_method!(into_str(self) { Self::Str as (a): (String) });

    pub fn into_number(self) -> Result<NumberValue> {
        match self {
            Self::Integer(num) => Ok(num.into()),
            Self::Float(num) => Ok(num.into()),
            _ => Err(Error::TypeError {
                expected: "number".into(),
                found: format!("{:?}", self),
            }),
        }
    }
}

impl From<Literal> for Value {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Nil => Self::Nil,
            Literal::Bool(val) => Self::Bool(val),
            Literal::Integer(val) => Self::Integer(val),
            Literal::Float(val) => Self::Float(val),
            Literal::Str(val) => Self::Str(val),
        }
    }
}

macro_rules! from_host_val_impl {
    (From<$host:ty> for $enum:ident :: $var:ident) => {
        impl From<$host> for $enum {
            fn from(host: $host) -> Self {
                Self::$var(host)
            }
        }
    };
}

from_host_val_impl!(From<bool> for Value::Bool);
from_host_val_impl!(From<isize> for Value::Integer);
from_host_val_impl!(From<f64> for Value::Float);
from_host_val_impl!(From<String> for Value::Str);

/// A wrapper around [`Value`] providing reference counting and shared mutability
#[derive(Clone, PartialEq)]
pub struct ValueWrap {
    inner: Rc<RefCell<Value>>,
}

impl ValueWrap {
    pub fn wrapping(val: Value) -> Self {
        Self {
            inner: Rc::new(RefCell::new(val)),
        }
    }

    pub fn try_unwrap(self) -> std::result::Result<Value, Self> {
        Rc::try_unwrap(self.inner)
            .map_err(|rc| Self { inner: rc })
            .map(RefCell::into_inner)
    }
}

impl ops::Deref for ValueWrap {
    type Target = RefCell<Value>;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl fmt::Debug for ValueWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("ValueWrap")
            .field(&self.inner.borrow())
            .finish()
    }
}

/// A subset of values which can be ordered
#[derive(Debug, Clone, PartialEq)]
pub enum NumberValue {
    Integer(isize),
    Float(f64),
}

impl From<NumberValue> for Value {
    fn from(nv: NumberValue) -> Self {
        match nv {
            NumberValue::Integer(n) => Self::Integer(n),
            NumberValue::Float(n) => Self::Float(n),
        }
    }
}

from_host_val_impl!(From<isize> for NumberValue::Integer);
from_host_val_impl!(From<f64> for NumberValue::Float);

impl cmp::PartialOrd for NumberValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match *self {
            Self::Integer(a) => match *other {
                Self::Integer(b) => a.partial_cmp(&b),
                Self::Float(b) => (a as f64).partial_cmp(&b),
            },
            Self::Float(a) => match *other {
                Self::Integer(b) => a.partial_cmp(&(b as f64)),
                Self::Float(b) => a.partial_cmp(&b),
            },
        }
    }
}

macro_rules! numbervalue_op_impl {
    ($( $trait:ident )::+, $method:ident) => {
        impl $( $trait )::* for NumberValue {
            type Output = Self;
            fn $method(self, other: Self) -> Self {
                match self {
                    Self::Integer(a) => match other {
                        Self::Integer(b) => $( $trait )::*::$method(a, b).into(),
                        Self::Float(b) => $( $trait )::*::$method(a as f64, b).into(),
                    }
                    Self::Float(a) => match other {
                        Self::Integer(b) => $( $trait )::*::$method(a, b as f64).into(),
                        Self::Float(b) => $( $trait )::*::$method(a, b).into(),
                    }
                }
            }
        }
    }
}

numbervalue_op_impl!(ops::Add, add);
numbervalue_op_impl!(ops::Sub, sub);
numbervalue_op_impl!(ops::Mul, mul);
numbervalue_op_impl!(ops::Div, div);
