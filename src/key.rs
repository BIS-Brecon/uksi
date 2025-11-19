use std::ops::Deref;

use fstr::FStr;
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo},
};

/// A 16 character ID matching NBN naming conventions.
#[derive(Debug, PartialEq)]
pub struct NBNKey(pub(crate) FStr<16>);

impl Deref for NBNKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Type<Postgres> for NBNKey {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for NBNKey {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as Encode<Postgres>>::encode(&*self.0, buf)
    }
}

impl Decode<'_, Postgres> for NBNKey {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as Decode<Postgres>>::decode(value)?;

        Ok(Self(value.parse()?))
    }
}
