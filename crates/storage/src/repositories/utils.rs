use std::str::FromStr;

use bigdecimal::BigDecimal;

use crate::error::*;

pub fn decimal_from_str(value: impl AsRef<str>) -> StorageResult<BigDecimal> {
    BigDecimal::from_str(value.as_ref())
        .map_err(|_| StorageError::InvalidDecimal(value.as_ref().to_owned()))
}
