use std::convert::TryInto;
use std::ops::Deref;

use bytes::Bytes;
use failure::Error;

use crate::serialize::Serialize;

pub use self::boolean::Boolean;
pub use self::integer::Integer;
pub use self::map::Map;
pub use self::null::Null;
pub use self::string::String;

mod boolean;
mod integer;
mod map;
mod null;
mod string;

pub trait Value {
    fn get_marker(&self) -> Result<u8, Error>;
}

impl Value for Box<dyn Value> {
    fn get_marker(&self) -> Result<u8, Error> {
        self.deref().get_marker()
    }
}

impl Serialize for Box<dyn Value> {}

impl TryInto<Bytes> for Box<dyn Value> {
    type Error = Error;

    fn try_into(self) -> Result<Bytes, Self::Error> {
        self.try_into_bytes()
    }
}
