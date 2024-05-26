use ::k256::elliptic_curve::ff::PrimeField;
use anyhow::anyhow;
use ethers::prelude::*;
use ethers::{core::k256::elliptic_curve::Field, prelude::coins_bip39::English};
use std::fs;
use std::path::Path;

use crate::k256::ecdsa::SigningKey;
use crate::k256::{AffinePoint, Scalar, Secp256k1};
use crate::types::transaction::eip2718::TypedTransaction;
use crate::types::transaction::eip712::Eip712;

pub fn keypair_gen() -> anyhow::Result<SigningKey> {
    Ok(SigningKey::random(&mut rand::thread_rng()))
}

pub fn keypair_from_hex(s: &str) -> anyhow::Result<SigningKey> {
    SigningKey::from_slice(hex::decode(s)?.as_slice())
        .map_err(|e| anyhow!("error parsing hex: {e}"))
}

// pub fn keypair_from_bip39(phrase: &str) -> anyhow::Result<(Scalar<Secp256k1>, Point<Secp256k1>)> {
//     let sk_bytes = MnemonicBuilder::<English>::default()
//         .phrase(phrase)
//         .build()
//         .map_err(|e| anyhow!("error parsing mnemonic: {e}"))?
//         .signer()
//         .to_bytes();
//     let sk = Scalar::from_bytes(sk_bytes.as_slice()).unwrap();
//     let pk = Point::generator() * &sk;
//     Ok((sk, pk))
// }

pub fn write_to_keystore<D: AsRef<Path>, S: AsRef<str>, P: AsRef<[u8]>>(
    sk: SigningKey,
    dir: D,
    name: S,
    password: P,
) -> anyhow::Result<()> {
    let _ = fs::create_dir_all(&dir);
    eth_keystore::encrypt_key(
        dir,
        &mut rand::thread_rng(),
        &*sk.to_bytes(),
        password,
        Some(name.as_ref()),
    )
    .map_err(|e| anyhow!("error encrypting key: {e}"))
    .map(|_| ())
}

pub fn read_from_keystore<P: AsRef<Path>, S: AsRef<[u8]>>(
    path: P,
    password: S,
) -> anyhow::Result<(LocalWallet, Vec<u8>)> {
    let sk_bytes = eth_keystore::decrypt_key(path, password)?;
    let sk = SigningKey::from_slice(sk_bytes.as_slice())
        .map_err(|e| anyhow!("error parsing key: {e}"))?;
    Ok((LocalWallet::from(sk), sk_bytes))
}
