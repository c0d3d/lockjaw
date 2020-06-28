use base64;

struct KeyRingSID(); // TODO

pub struct Key {
    name: String,
    enc_data: Vec<u8>,
    keyring_sid: Option<KeyRingSID>,
}

impl Key {
    pub fn from_line(line: &String) -> Result<Key, &'static str> {
        let sections: Vec<_> = line.splitn(2, '\t').collect();

        if sections.len() != 2 {
            return Err("Bad line!");
        }

        let name = sections[0].to_string();

        return base64::decode(sections[1])
            .map(|bytes| Key {
                name,
                enc_data: bytes,
                keyring_sid: None,
            })
            .or(Err("Bad base64!"));
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
