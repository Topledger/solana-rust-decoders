pub mod pubkey_serde {
    use anchor_lang::prelude::Pubkey;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialize a Pubkey as base58 string
    pub fn serialize<S>(key: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&key.to_string())
    }

    // Deserialize a base58 string back into Pubkey
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Pubkey::from_str(&s).map_err(DeError::custom)
    }
}

pub mod pubkey_serde_option {
    use super::pubkey_serde;
    use anchor_lang::prelude::Pubkey;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(opt: &Option<Pubkey>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(key) => serializer.serialize_some(&key.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Pubkey>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => Ok(Some(Pubkey::from_str(&s).map_err(DeError::custom)?)),
            None => Ok(None),
        }
    }
} 