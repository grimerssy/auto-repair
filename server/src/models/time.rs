use crate::errors::Error;
use core::fmt;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql},
    sql_types,
};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

static MINUTE: i64 = 60_000_000;
static HOUR: i64 = 60 * MINUTE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Time)]
pub struct Time(i64);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time = self.0 % (24 * HOUR);
        let hours = time / HOUR;
        let minutes = (time - hours * HOUR) / MINUTE;
        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = "invalid time format".to_string();
        let split = s.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(Error::BadRequest(err_msg));
        }
        let hours = split[0]
            .parse::<i64>()
            .map_err(|_| Error::BadRequest(err_msg.clone()))?;
        let minutes = split[1]
            .parse::<i64>()
            .map_err(|_| Error::BadRequest(err_msg.clone()))?;
        Ok(Time(hours * HOUR + minutes * MINUTE))
    }
}

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
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        Time::from_str(&str).map_err(D::Error::custom)
    }
}
