use hex::FromHexError;
use serde::Serialize;
use std::str::FromStr;

pub enum Error {
    /// An address contained an invalid hexadecimal digit.
    Hex,
    /// An address was not of the expected length.
    Len,
}

impl From<FromHexError> for Error {
    fn from(_: FromHexError) -> Self {
        Error::Hex
    }
}

// Parsing internally allocates a `Vec<u8>` and then calls `try_into`, which returns `Err(Vec<u8>)`
// if the parsing fails.  We will update our `FromStr` imp shortly to avoid the allocation.
impl From<Vec<u8>> for Error {
    fn from(_: Vec<u8>) -> Self {
        Error::Len
    }
}

pub type Result<T> = std::result::Result<T, Error>;

const SUI_ADDRESS_LENGTH: usize = 32;

/// Uniquely identifies an account on a particular blockchain.
#[derive(Serialize)]
pub struct Address([u8; SUI_ADDRESS_LENGTH]);

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // The logic here is specific to Sui, but may be extended in the future to support other
        // chains.  See also: <https://docs.sui.io/guides/developer/getting-started/get-address>

        // TODO: Elide `Vec` allocation by `hex` crate.  We know the signature length in advance,
        //  and return an error if the input is malformed.  We should not need third-party code to
        //  parse hex.
        let digits = s.trim().strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(digits)?;
        Ok(Address(bytes.try_into()?))
    }
}
