pub mod pubkey_serde {
    use bs58;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};

    // serialize &[u8;32] as a base58 &str
    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bs58::encode(bytes).into_string();
        serializer.serialize_str(&s)
    }

   
   
}

pub mod pubkey_serde_option {
    use bs58;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(opt: &Option<[u8; 32]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(bytes) => serializer.serialize_some(&bs58::encode(bytes).into_string()),
            None => serializer.serialize_none(),
        }
    }

   
} 