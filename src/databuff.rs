use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

pub const MAX_PAYLOAD_SIZE: usize = 4096;

#[derive(Clone, Copy)]
pub struct DataBuff(pub [u8; MAX_PAYLOAD_SIZE]);

impl DataBuff {
    pub fn from_slice(data: &[u8]) -> Option<DataBuff> {
        let mut b = [0; MAX_PAYLOAD_SIZE];

        if data.len() > MAX_PAYLOAD_SIZE {
            return None;
        }

        b[..data.len()].copy_from_slice(data);

        return Some(DataBuff(b));
    }
}

impl AsRef<[u8]> for DataBuff {
    fn as_ref(&self) -> &[u8] {
        return &self.0;
    }
}

impl Deref for DataBuff {
    type Target = [u8; MAX_PAYLOAD_SIZE];

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for DataBuff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Serialize for DataBuff {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_bytes(&self.0);
    }
}

struct FourKVisitor;

impl<'de> Visitor<'de> for FourKVisitor {
    type Value = [u8; MAX_PAYLOAD_SIZE];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("4K Buffer")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut buff = [0; MAX_PAYLOAD_SIZE];
        if v.len() < MAX_PAYLOAD_SIZE {
            return Err(E::custom(format!("Buffer too short {}!", v.len())));
        }
        unsafe {
            std::intrinsics::copy_nonoverlapping(v.as_ptr(), buff.as_mut_ptr(), MAX_PAYLOAD_SIZE);
        }

        return Ok(buff);
    }
}

impl<'de> Deserialize<'de> for DataBuff {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        return Ok(DataBuff(deserializer.deserialize_bytes(FourKVisitor)?));
    }
}

impl Default for DataBuff {
    fn default() -> DataBuff {
        return DataBuff([0; MAX_PAYLOAD_SIZE]);
    }
}
