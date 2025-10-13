use lume_utils::b64::b64u_encode;
use rand::RngCore;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<()> {
    let mut key = [0u8; 64];
    rand::rng().fill_bytes(&mut key);
    println!("\nGenerated with rand::rng() {key:?}");

    let b64u = b64u_encode(key);
    println!("\nGenerated with b64u encode: {b64u:?}");

    Ok(())
}
