use argon2::{
    password_hash::rand_core::OsRng, password_hash::SaltString, Algorithm, Argon2, ParamsBuilder,
    PasswordHash, PasswordHasher, PasswordVerifier, Version,
};

#[derive(Clone)]
pub struct HashService<'a> {
    pub ctx: Argon2<'a>,
}

impl<'a> HashService<'a> {
    pub fn new(secret: &'a [u8]) -> Self {
        return Self {
            ctx: Argon2::new_with_secret(
                &secret,
                Algorithm::Argon2i,
                Version::V0x13,
                ParamsBuilder::new()
                    .m_cost(65)
                    .t_cost(3)
                    .p_cost(4)
                    .output_len(32)
                    .build()
                    .unwrap(),
            )
            .unwrap(),
        };
    }

    pub fn hash(&self, password: &str) -> String {
        let pwd = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        return self.ctx.hash_password(pwd, &salt).unwrap().to_string();
    }

    pub fn verify(&self, password: &str, hash: &str) -> bool {
        let pwd = password.as_bytes();
        let phash = PasswordHash::new(hash).unwrap();
        return match self.ctx.verify_password(pwd, &phash) {
            Ok(_) => true,
            Err(_) => false,
        };
    }
}
