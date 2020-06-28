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
            for line in read_lines(path)? {
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
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
