use crate::key::Key;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Keystore {
    loc: String,
    secrets: HashMap<String, Key>,
}

enum LoadFailure {
    KeyFileNotFound,
    KeyFileMalformed,
    KeyFileUnreadable,
    Other(io::ErrorKind),
}

impl From<io::Error> for LoadFailure {
    fn from(e: io::Error) -> LoadFailure {
        return match e.kind() {
            io::ErrorKind::NotFound => LoadFailure::KeyFileNotFound,
            x => LoadFailure::Other(x),
        };
    }
}

impl From<&'static str> for LoadFailure {
    fn from(_: &'static str) -> LoadFailure {
        LoadFailure::KeyFileMalformed
    }
}

impl Keystore {
    pub fn load<T: AsRef<Path>>(path: &T) -> Result<Keystore, LoadFailure> {
        if let Some(p_str) = path.as_ref().to_str() {
            let mut secrets = HashMap::new();
            let file = File::open(path)?;
            for line in io::BufReader::new(file).lines() {
                let k = Key::from_line(&line?)?;
                secrets.insert(k.get_name().clone(), k);
            }
            return Ok(Keystore {
                loc: p_str.to_string(),
                secrets,
            });
        } else {
            return Err(LoadFailure::KeyFileUnreadable);
        }
    }

    pub fn find_key<T>(&self, name: T) -> Option<&Key>
    where
        T: AsRef<str>,
    {
        self.secrets.get(name.as_ref())
    }
}
