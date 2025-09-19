pub mod pubkey_serde {
    use bs58;
    use serde::de::Error as DeError;
    use serde::{Deserialize, Deserializer, Serializer};
    use crate::pubkey_serde;
    use crate::pubkey_serde_option;


    // serialize &[u8;32] as a base58 &str
    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bs58::encode(bytes).into_string();
        serializer.serialize_str(&s)
    }

    // deserialize a base58 &str to [u8;32]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let vec = bs58::decode(&s).into_vec().map_err(DeError::custom)?;
        if vec.len() != 32 {
            return Err(DeError::invalid_length(vec.len(), &"expected 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&vec);
        Ok(arr)
    }
}

pub mod pubkey_serde_option {
    use super::pubkey_serde;
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<[u8; 32]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = opt {
            let vec = bs58::decode(&s).into_vec().map_err(DeError::custom)?;
            if vec.len() != 32 {
                return Err(DeError::invalid_length(vec.len(), &"expected 32 bytes"));
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&vec);
            Ok(Some(arr))
        } else {
            Ok(None)
        }
    }
}
