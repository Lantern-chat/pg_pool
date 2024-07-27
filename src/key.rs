use std::{
    any::{Any, TypeId},
    borrow::{Borrow, Cow},
};

use tokio_postgres::types::Type;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct StatementCacheKeyedKey<'a> {
    pub query: Cow<'a, str>,
    pub types: Cow<'a, [Type]>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum StatementCacheKey<'a> {
    Typed(TypeId),
    Keyed(StatementCacheKeyedKey<'a>),
}

#[repr(transparent)]
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct StaticStatementCacheKey(pub StatementCacheKey<'static>);

impl StaticStatementCacheKey {
    #[inline]
    pub fn owned(query: String, types: Vec<Type>) -> StaticStatementCacheKey {
        StaticStatementCacheKey(StatementCacheKey::Keyed(StatementCacheKeyedKey {
            query: Cow::Owned(query),
            types: Cow::Owned(types),
        }))
    }

    pub fn typed<T: Any>() -> StaticStatementCacheKey {
        StaticStatementCacheKey(StatementCacheKey::Typed(TypeId::of::<T>()))
    }
}

impl<'a> Borrow<StatementCacheKey<'a>> for StaticStatementCacheKey {
    #[inline(always)]
    fn borrow(&self) -> &StatementCacheKey<'a> {
        // SAFETY: Borrowing for any sub-lifetime 'a
        // is valid for a 'static borrow, just not
        // the other way around.
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> StatementCacheKey<'a> {
    #[inline(always)]
    pub const fn borrowed(query: &'a str, types: &'a [Type]) -> StatementCacheKey<'a> {
        StatementCacheKey::Keyed(StatementCacheKeyedKey {
            query: Cow::Borrowed(query),
            types: Cow::Borrowed(types),
        })
    }

    pub fn typed<T: Any>() -> StatementCacheKey<'a> {
        StatementCacheKey::Typed(TypeId::of::<T>())
    }
}
