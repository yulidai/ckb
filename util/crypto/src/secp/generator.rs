use super::privkey::Privkey;
use super::pubkey::Pubkey;
use super::SECP256K1;
use rand::{self, Rng};
use secp256k1::{PublicKey, SecretKey};

pub struct Generator;

impl Generator {
    pub fn random_privkey() -> Privkey {
        Self::random_secret_key().into()
    }

    pub fn random_keypair() -> (Privkey, Pubkey) {
        let secret_key = Self::random_secret_key();
        let pubkey = PublicKey::from_secret_key(&*SECP256K1, &secret_key);

        (secret_key.into(), pubkey.into())
    }

    pub fn random_secret_key() -> SecretKey {
        let mut seed = vec![0; 32];
        let mut rng = rand::thread_rng();
        loop {
            rng.fill(seed.as_mut_slice());
            if let Ok(key) = SecretKey::from_slice(&seed) {
                return key;
            }
        }
    }
}
