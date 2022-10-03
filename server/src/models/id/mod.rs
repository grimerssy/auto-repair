pub mod keys;

use core::fmt;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::{deserialize, sql_types, FromSqlRow};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

use crate::errors::ServerError;

use self::keys::Key;

static DIGITS: &[u8] = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ".as_bytes();

fn init_from_digit() -> impl Fn(u8) -> Result<i32, ServerError> {
    let map = DIGITS
        .iter()
        .enumerate()
        .map(|(i, e)| (*e, i as i32))
        .collect::<HashMap<_, _>>();

    move |d| {
        map.get(&d)
            .copied()
            .ok_or_else(|| ServerError::FailToParse("invalid id digit".into()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Serial)]
pub enum Id {
    Raw(i32),
    Encoded(i32),
}

impl Id {
    pub fn encode(&mut self, key: Key) {
        *self = match self {
            Id::Raw(n) => Id::Encoded(((*n * key.prime) & i32::MAX) ^ key.random),
            Id::Encoded(_) => panic!("Cannot encode encoded id"),
        }
    }
    pub fn decode(&mut self, key: Key) {
        *self = match self {
            Id::Raw(_) => panic!("Cannot decode raw id"),
            Id::Encoded(n) => Id::Raw(((*n ^ key.random) * key.mod_inverse) & i32::MAX),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value = match self {
            Id::Raw(_) => panic!("Cannot display raw id"),
            Id::Encoded(value) => *value,
        };
        if value < 1 {
            panic!("invalid id value")
        }
        let radix = DIGITS.len() as i32;
        const LEN: usize = 6;
        let mut result = [0_u8; LEN];
        for c in result.iter_mut().rev() {
            *c = DIGITS[(value % radix) as usize];
            value /= radix;
        }
        let mut meet_non_zero = false;
        for c in result.into_iter() {
            if c != DIGITS[0] {
                meet_non_zero = true;
            }
            if meet_non_zero {
                write!(f, "{}", c as char)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Id {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const LEN: usize = 6;
        if s.len() > LEN {
            return Err(ServerError::FailToParse("invalid id format".into()));
        }
        let radix = DIGITS.len() as i32;
        let mut result = 0;
        let mut multiplier = 1;
        let from_digit = init_from_digit();
        for d in s.bytes().rev() {
            let digit = from_digit(d)?;
            result += digit * multiplier;
            multiplier *= radix;
        }
        Ok(Id::Encoded(result))
    }
}

impl ToSql<sql_types::Serial, Pg> for Id {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            Id::Raw(value) => ToSql::<sql_types::Serial, Pg>::to_sql(value, out),
            Id::Encoded(_) => panic!("Cannot use encoded id in database"),
        }
    }
}

impl FromSql<sql_types::Serial, Pg> for Id {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        FromSql::<sql_types::Serial, Pg>::from_sql(bytes).map(Id::Raw)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Id, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = &String::deserialize(deserializer)?;
        Id::from_str(str).map_err(D::Error::custom)
    }
}
