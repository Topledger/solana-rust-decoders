pub mod pubkey_serde {
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&bs58::encode(bytes).into_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        let b = bs58::decode(s)
            .into_vec()
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(<[u8; 32]>::try_from(b.as_slice()).map_err(|_| serde::de::Error::custom("invalid length"))?)
    }
}

pub mod pubkey_serde_option {
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(opt: &Option<[u8; 32]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(arr) => serializer.serialize_str(&bs58::encode(arr).into_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<[u8; 32]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = serde::Deserialize::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                let b = bs58::decode(s)
                    .into_vec()
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(Some(<[u8; 32]>::try_from(b.as_slice()).map_err(|_| serde::de::Error::custom("invalid length"))?))
            }
            None => Ok(None),
        }
    }
} 