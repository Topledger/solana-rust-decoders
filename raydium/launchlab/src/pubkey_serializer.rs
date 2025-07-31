pub mod pubkey_serde {
    use anchor_lang::prelude::Pubkey;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // serialize Pubkey as a base58 string
    pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = pubkey.to_string();
        serializer.serialize_str(&s)
    }

    // deserialize a base58 string to Pubkey
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Pubkey::from_str(&s).map_err(DeError::custom)
    }
}

pub mod pubkey_serde_option {
    use anchor_lang::prelude::Pubkey;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(opt: &Option<Pubkey>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(pubkey) => serializer.serialize_some(&pubkey.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Pubkey>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = opt {
            let pubkey = Pubkey::from_str(&s).map_err(DeError::custom)?;
            Ok(Some(pubkey))
        } else {
            Ok(None)
        }
    }
} 