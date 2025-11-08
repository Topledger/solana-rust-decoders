pub mod pubkey_serde {
    use bs58;
    use serde::{Serializer};
    use anchor_lang::prelude::Pubkey;

    // serialize &Pubkey as a base58 string
    pub fn serialize<S>(bytes: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bytes.to_string();
        serializer.serialize_str(&s)
    }
}

pub mod pubkey_serde_option {
    use serde::{Serializer};
    use anchor_lang::prelude::Pubkey;

    pub fn serialize<S>(opt: &Option<Pubkey>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(bytes) => serializer.serialize_some(&bytes.to_string()),
            None => serializer.serialize_none(),
        }
    }
}

