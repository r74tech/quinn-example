use anyhow::Result;
mod common;

fn main() -> Result<()> {
    let (cert_der, key_der) = common::generate_self_signed_cert()?;
    common::save_cert_and_key(&cert_der, &key_der)?;
    Ok(())
}