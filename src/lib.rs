use serde::{Serializer, Serialize, ser};
use std::io::Write;
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoronaError {
    #[error("Custom error: {0}")]
    Custom(String)
}
pub type Result<T> = std::result::Result<T, CoronaError>;

impl ser::Error for CoronaError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

struct CoronaSerializer<'a, W: Write + 'a> {
    paths: HashMap<&'static str, &'static str>,
    output: &'a mut W,
}

pub fn write<T: Serialize, W: Write>(from: &T, to: &mut W) {
    unimplemented!();
}

pub fn make_string<T: Serialize>(from: &T) -> String {
    unimplemented!();
}

impl<'a, W: Write + 'a> Serializer for CoronaSerializer<'a, W> {
    type Ok = ();
    type Error = CoronaError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.write_all(if v { "true" } else { "false" }.as_bytes());
        Ok(())
    }
}
