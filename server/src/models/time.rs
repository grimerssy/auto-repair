use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Time)]
pub struct Time(i64);

impl ToSql<sql_types::Time, Pg> for Time {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<sql_types::BigInt, Pg>::to_sql(&self.0, out)
    }
}

impl FromSql<sql_types::Time, Pg> for Time {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        FromSql::<sql_types::BigInt, Pg>::from_sql(bytes).map(Time)
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let microseconds = i64::deserialize(deserializer)?;
        Ok(Time(microseconds))
    }
}
