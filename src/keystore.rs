use crate::key;

use sodiumoxide::crypto::secretbox;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Keystore {
    keys: HashMap<String, key::RawKey>,
}
