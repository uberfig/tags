use serde::{de::Error as DeError, ser::Error as SerError, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::cryptography::key::Key;
use crate::cryptography::universal_keys::UniversalPublic;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApPublicKey {
    pub id: Url,    //https://my-example.com/actor#main-key
    pub owner: Url, //"https://my-example.com/actor"
    #[serde(deserialize_with = "deserialize_public")]
    #[serde(serialize_with = "serialize_public")]
    pub public_key_pem: UniversalPublic,
}

pub fn deserialize_public<'de, D>(deserializer: D) -> Result<UniversalPublic, D::Error>
where
    D: Deserializer<'de>,
{
    let input = String::deserialize(deserializer)?;
    match UniversalPublic::from_pem(&input) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(D::Error::custom(err)),
    }
}

pub fn serialize_public<S>(x: &UniversalPublic, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pem = x.to_pem();
    match pem {
        Ok(ok) => s.serialize_str(&ok),
        Err(x) => Err(S::Error::custom(x)),
    }
}

impl From<String> for ApPublicKey {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon
        let pub_key = r#"
{
    "id": "https://mastodon.social/users/Mastodon#main-key",
    "owner": "https://mastodon.social/users/Mastodon",
    "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtpNfuGPl/WTnSq3dTurF\nMRelAIdvGVkO/VKYZJvIleYA27/YTnpmlY2g+0az4xEhOBtVNA1cTpS63CdXRyNz\ncH/GZtzxkdxN91vZSw0JVy+wG34dzwcq1KWFDz9D/5Tqf16KUJH+TDTlxdOBds91\nIZg+TTkiT+xfnSiC5SLMnn1dTzCW9P0yNJxpn37z7p6pEs63X1wstEEX1qGDUQTO\n1JICpKDjuQZMlioAAA5KG25tg2f+zKlv5M/NI33DblquyJ7TYvIpDN8hsFCRjuvA\nmjtKz/1XIRvQkeKND3UkqX8s6qTGyNOjcT86qt9BqYHYGuppjpRG/QNGoKYalio1\nwwIDAQAB\n-----END PUBLIC KEY-----\n"
}
        "#;
        let deserialized: Result<ApPublicKey, serde_json::Error> = serde_json::from_str(pub_key);

        match deserialized {
            Ok(_x) => Ok(()),
            Err(x) => Err(format!("pub key failed: {}", x)),
        }
    }
}
