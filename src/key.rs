use crate::memlock::MLock;
use base64;

type StrErr<T> = Result<T, &'static str>;
type LockedBuff = MLock<[u8; 4096]>;

#[derive(Clone)]
pub enum KeyType {
    OTC,
    PASSWORD,
}

impl KeyType {
    fn from_string<T: AsRef<str>>(s: T) -> StrErr<KeyType> {
        return match s.as_ref() {
            "OTC" => Ok(KeyType::OTC),
            "PASSWORD" => Ok(KeyType::PASSWORD),
            _ => Err("Unrecognized key type!"),
        };
    }
}

impl Into<String> for KeyType {
    fn into(self) -> String {
        return match self {
            KeyType::OTC => "OTC".to_string(),
            KeyType::PASSWORD => "PASSWORD".to_string(),
        };
    }
}

pub struct PlainKey {
    name: String,
    ktype: KeyType,
    enc_data: Vec<u8>,
}

pub struct Key {
    name: String,
    ktype: KeyType,
    enc_data: LockedBuff,
}

impl PlainKey {
    pub fn from_line(line: &String) -> StrErr<PlainKey> {
        return KeyStoreLine::parse(line)?.into();
    }

    pub fn to_line(&self) -> String {
        let ks: KeyStoreLine<_> = self.into();
        return ks.into();
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}

struct KeyStoreLine<T>
where
    T: AsRef<str>,
{
    name: T,
    ktype: T,
    enc_data: T,
}

impl<'a> KeyStoreLine<&'a str> {
    const LINE_ENTRIES_COUNT: usize = 3;
    const LINE_ENTRY_SEP: char = '\t';

    fn parse<T>(line: &'a T) -> StrErr<KeyStoreLine<&'a str>>
    where
        T: AsRef<str>,
    {
        return match &*line
            .as_ref()
            .splitn(
                KeyStoreLine::LINE_ENTRIES_COUNT,
                KeyStoreLine::LINE_ENTRY_SEP,
            )
            .collect::<Vec<_>>()
        {
            &[name, ktype, enc_data] => Ok(KeyStoreLine {
                name,
                ktype,
                enc_data,
            }),
            _ => Err("Bad entry count!"),
        };
    }
}

impl From<&PlainKey> for KeyStoreLine<String> {
    fn from(k: &PlainKey) -> KeyStoreLine<String> {
        return KeyStoreLine {
            name: k.name.clone(),
            ktype: k.ktype.clone().into(),
            enc_data: base64::encode(&k.enc_data),
        };
    }
}

impl<T> From<KeyStoreLine<T>> for String
where
    T: AsRef<str>,
{
    fn from(ksl: KeyStoreLine<T>) -> String {
        return format!(
            "{}\t{}\t{}",
            ksl.name.as_ref(),
            ksl.ktype.as_ref(),
            ksl.enc_data.as_ref()
        );
    }
}

impl<T> From<KeyStoreLine<T>> for StrErr<PlainKey>
where
    T: AsRef<str>,
{
    fn from(ksl: KeyStoreLine<T>) -> StrErr<PlainKey> {
        let name = ksl.name.as_ref().to_string();
        let ktype = KeyType::from_string(ksl.ktype)?;
        let enc_data = base64::decode(ksl.enc_data.as_ref()).or(Err("Bad base64!"))?;
        return Ok(PlainKey {
            name,
            ktype,
            enc_data,
        });
    }
}
