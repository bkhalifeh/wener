use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    id: i32,
}

pub struct JwtService {
    encodingKey: EncodingKey,
    decodingKey: DecodingKey,
}

impl JwtService {
    fn new(secret: &str) -> Self {
        return Self {
            encodingKey: EncodingKey::from_secret(secret.as_bytes()),
            decodingKey: DecodingKey::from_secret(secret.as_bytes()),
        };
    }

    fn sign(&self, id: i32) -> String {
        let my_claims = Claims {
            id,
            exp: (get_current_timestamp() as usize) * 7 * 24 * 3600,
        };
        let out = encode(&Header::default(), &my_claims, &self.encodingKey).unwrap();
        return out;
    }

    fn verify(&self, token: &str) -> Claims {
        let out = decode::<Claims>(token, &self.decodingKey, &Validation::default()).unwrap();
        return out.claims;
    }
}
