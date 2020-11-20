use std::marker::PhantomData;

use crate::ast::value::StoredNonPrimitive;
use crate::ast::value::Value;
use crate::chain::ergo_box::ErgoBox;
use crate::types::stype::LiftIntoSType;

pub trait STypeT {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SBoxT {}
impl STypeT for SBoxT {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SIntT {}
impl STypeT for SIntT {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SCollT<T: STypeT> {
    p: PhantomData<T>,
}

impl<T: STypeT> STypeT for SCollT<T> {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TValue<T: STypeT> {
    pub v: Value,
    p: PhantomData<T>,
}

impl From<ErgoBox> for TValue<SBoxT> {
    fn from(b: ErgoBox) -> Self {
        TValue {
            v: b.into(),
            p: PhantomData,
        }
    }
}

impl From<i32> for TValue<SIntT> {
    fn from(v: i32) -> Self {
        TValue {
            v: v.into(),
            p: PhantomData,
        }
    }
}

impl<T: LiftIntoSType + StoredNonPrimitive + Into<Value>, S: STypeT> From<Vec<T>> for TValue<S> {
    fn from(raw: Vec<T>) -> Self {
        TValue {
            v: raw.into(),
            p: PhantomData,
        }
    }
}
