use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct DataBuff(pub [u8; 4096]);

impl AsRef<[u8]> for DataBuff {
    fn as_ref(&self) -> &[u8] {
        return &self.0;
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
    type Value = [u8; 4096];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("4K Buffer")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut buff = [0; 4096];
        if v.len() < 4096 {
            return Err(E::custom(format!("Buffer too short {}!", v.len())));
        }
        unsafe {
            std::intrinsics::copy_nonoverlapping(v.as_ptr(), buff.as_mut_ptr(), 4096);
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
        return DataBuff([0; 4096]);
    }
}
