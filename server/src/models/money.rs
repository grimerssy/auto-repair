use std::ops::{Add, AddAssign, Sub, SubAssign};

use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{self, BigInt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Money)]
pub struct Money(i64);

impl FromSql<sql_types::Money, Pg> for Money {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        FromSql::<BigInt, Pg>::from_sql(bytes).map(Money)
    }
}

impl ToSql<sql_types::Money, Pg> for Money {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<BigInt, Pg>::to_sql(&self.0, out)
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.0 as f32 / 100_f32)
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Money, D::Error>
    where
        D: Deserializer<'de>,
    {
        let money = f32::deserialize(deserializer)?;
        Ok(Money((money * 100_f32) as i64))
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, rhs: Money) -> Self::Output {
        self.0
            .checked_add(rhs.0)
            .map(Money)
            .expect("overflow adding money amounts")
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.0 = self
            .0
            .checked_add(rhs.0)
            .expect("overflow adding money amounts")
    }
}

impl Sub for Money {
    type Output = Self;
    fn sub(self, rhs: Money) -> Self::Output {
        self.0
            .checked_sub(rhs.0)
            .map(Money)
            .expect("underflow subtracting money amounts")
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Money) {
        self.0 = self
            .0
            .checked_sub(rhs.0)
            .expect("underflow subtracting money amounts")
    }
}
