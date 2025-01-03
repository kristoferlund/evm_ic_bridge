use alloy::{hex::FromHex, primitives::FixedBytes};
use anyhow::{anyhow, Result};

pub fn fixed_bytes_from_hex_str<const N: usize>(hex_string: &str) -> Result<FixedBytes<N>> {
    let stripped = hex_string
        .strip_prefix("0x")
        .ok_or(anyhow!("Invalid hex string"))?;
    FixedBytes::from_hex(stripped).map_err(|_| anyhow!("Invalid hex string"))
}

pub fn fixed_bytes_array_from_hex_str<const N: usize>(hex_string: &str) -> Result<[u8; N]> {
    let stripped = hex_string
        .strip_prefix("0x")
        .ok_or(anyhow!("Invalid hex string"))?;
    <[u8; N]>::from_hex(stripped).map_err(|_| anyhow!("Invalid hex string"))
}
