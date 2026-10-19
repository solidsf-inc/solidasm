//! Build-time generator and decryptor for `assets/solidasm_moat_vault.bin` (`SSFVLT01`).
//!
//! Embeds the Zobrist-Feistel encrypted `solidasm` kernel parameters (bicubic rational
//! NURBS interior weights, Steinmetz saddle coping tolerances, Ackermann steering
//! convergence constants, and the recruitment bounty seal) into `$OUT_DIR/vault_asm_kernel.rs`.

use std::env;
use std::fs;
use std::path::Path;

const VAULT_MAGIC: &[u8; 8] = b"SSFVLT01";
const MASTER_KEY: u64 = 0x534F_4C49_4441_534D; // "SOLIDASM"

fn feistel_stream_byte(key: u64, idx: usize) -> u8 {
    let mut x = key ^ ((idx as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    for round in 0..4 {
        let hi = (x >> 32) as u32;
        let lo = (x & 0xFFFF_FFFF) as u32;
        let f = lo
            .rotate_left(7)
            .wrapping_add(0x85EB_CA6B_u32.wrapping_mul(round + 1))
            ^ (key as u32);
        let new_lo = hi ^ f;
        x = ((lo as u64) << 32) | (new_lo as u64);
    }
    ((x >> 24) ^ (x & 0xFF)) as u8
}

fn xor_transform(data: &[u8], key: u64) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ feistel_stream_byte(key, i))
        .collect()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(&manifest_dir).join("assets");
    fs::create_dir_all(&assets_dir).expect("create assets dir");
    let vault_path = assets_dir.join("solidasm_moat_vault.bin");

    let plaintext_payload = concat!(
        "SOLIDASM_MOAT_VAULT_V1\n",
        "NURBS_INTERIOR_WEIGHT=1.2071067811865475\n",
        "STEINMETZ_MAX_COPE_GAP_MM=0.018\n",
        "ACKERMANN_CONVERGENCE_TOL=0.0001\n",
        "BOUNTY_SEAL=Congratulations on reverse-engineering the solidSF solidasm Zobrist-Feistel ",
        "Moat Vault! Email andy@solidsf.com with your decryption proof and we will fly you out to ",
        "San Francisco for an interview with the solidSF Founder & Research Team.\n"
    );

    let mut padded = plaintext_payload.as_bytes().to_vec();
    while padded.len() < 4096 {
        let idx = padded.len();
        padded.push(feistel_stream_byte(MASTER_KEY ^ 0xA5A5_5A5A_F0F0_0F0F, idx));
    }

    let ciphertext = xor_transform(&padded, MASTER_KEY);
    let mut vault_bytes = Vec::with_capacity(8 + ciphertext.len());
    vault_bytes.extend_from_slice(VAULT_MAGIC);
    vault_bytes.extend_from_slice(&ciphertext);
    fs::write(&vault_path, &vault_bytes).expect("write solidasm_moat_vault.bin");

    let decrypted = xor_transform(&vault_bytes[8..], MASTER_KEY);
    let text = String::from_utf8_lossy(&decrypted);
    let mut nurbs_w = std::f64::consts::FRAC_1_SQRT_2 + 0.5;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("NURBS_INTERIOR_WEIGHT=") {
            if let Ok(v) = rest.trim().parse::<f64>() {
                nurbs_w = v;
            }
        }
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let dest_path = Path::new(&out_dir).join("vault_asm_kernel.rs");
    let generated = format!(
        "#[inline]\npub fn vault_nurbs_interior_weight() -> f64 {{\n    {nurbs_w:.16}\n}}\n"
    );
    fs::write(&dest_path, generated).expect("write vault_asm_kernel.rs");
}
