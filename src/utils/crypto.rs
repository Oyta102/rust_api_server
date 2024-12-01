use md5::Md5;
use secp256k1::{rand, SecretKey};
use sha2::{Digest, Sha256};

pub fn hash_pass(password: &str, salt: &str) ->String{
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}",password,salt));
    let sha256_hash = hasher.finalize();
    let mut md5_hasher = Md5::new();
    md5_hasher.update(&sha256_hash);
    let md5_hash = md5_hasher.finalize();
    let md5_hex = md5_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    assert_eq!(md5_hex.len(), 32);
    md5_hex
}

pub fn verify_pass(password: &str, hashed_password:  &str, salt: &str)->bool{
    hash_pass(password,salt)  == hashed_password
}

pub fn generate_private_key()->String{
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    format!("{}",secret_key.display_secret())
}