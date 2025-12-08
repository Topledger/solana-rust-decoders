pub mod pubkey_serde {
    use anchor_lang::prelude::Pubkey;
    use serde::Serializer;

    // serialize Pubkey as a base58 string
    pub fn serialize<S>(bytes: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bytes.to_string();
        serializer.serialize_str(&s)
    }
}

pub mod pubkey_serde_u32 {
    use serde::Serializer;

    // serialize &[u8;32] as a base58 string
    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bs58::encode(bytes).into_string();
        serializer.serialize_str(&s)
    }
}

pub mod pubkey_serde_option {
    use anchor_lang::prelude::Pubkey;
    use serde::Serializer;

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
