//! Core types and canonical encoding for bitWASM receipts.

use sha2::{Digest, Sha256};

pub const BITWASM_VERSION_V0: u16 = 0;
pub const VM_ID_WASM32_BITWASM_V0: &str = "wasm32-bitwasm-v0";

pub const MAX_MEMORY_PAGES_V0: u32 = 256;
pub const MAX_STACK_DEPTH_V0: u32 = 1024;

pub const GAS_COST_INTEGER_ARITH_V0: u64 = 1;
pub const GAS_COST_MEMORY_LOAD_STORE_V0: u64 = 3;
pub const GAS_COST_CONTROL_FLOW_V0: u64 = 1;
pub const GAS_COST_FUNCTION_CALL_V0: u64 = 5;
pub const GAS_COST_MEMORY_GROW_V0: u64 = 50;

pub const HOST_FUNC_SHA256_V0: &str = "sha256";
pub const HOST_FUNC_VERIFY_SIGNATURE_V0: &str = "verify_signature";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptV0 {
    pub bitwasm_version: u16,
    pub vm_id: String,
    pub wasm_hash: [u8; 32],
    pub state_root_before: [u8; 32],
    pub input_hash: [u8; 32],
    pub state_root_after: [u8; 32],
    pub outputs_hash: [u8; 32],
    pub gas_used: u64,
    pub fee_sats: u64,
    pub expiry_height: u32,
    pub federation_id: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptError {
    VmIdTooLong { len: usize },
}

impl ReceiptV0 {
    /// Canonical serialization per WIP-0002.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ReceiptError> {
        serialize_receipt_v0(self)
    }

    pub fn hash(&self) -> Result<[u8; 32], ReceiptError> {
        let bytes = self.canonical_bytes()?;
        Ok(sha256(&bytes))
    }

    /// Structural checks only (lengths/encoding boundaries).
    pub fn is_well_formed(&self) -> bool {
        self.validate_well_formed().is_ok()
    }

    pub fn validate_well_formed(&self) -> Result<(), ReceiptError> {
        let vm_len = self.vm_id.as_bytes().len();
        if vm_len > u16::MAX as usize {
            return Err(ReceiptError::VmIdTooLong { len: vm_len });
        }
        Ok(())
    }

    pub fn hash_matches(&self, expected: [u8; 32]) -> Result<bool, ReceiptError> {
        Ok(self.hash()? == expected)
    }

    pub fn matches_outputs(&self, outputs_bytes: &[u8]) -> bool {
        outputs_hash(outputs_bytes) == self.outputs_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputsV0 {
    bytes: Vec<u8>,
}

impl OutputsV0 {
    /// Minimal v0 encoding: raw bytes. Higher-level schemas can wrap this later.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn hash(&self) -> [u8; 32] {
        outputs_hash(&self.bytes)
    }
}

pub fn input_hash(input_bytes: &[u8]) -> [u8; 32] {
    sha256(input_bytes)
}

pub fn outputs_hash(outputs_bytes: &[u8]) -> [u8; 32] {
    sha256(outputs_bytes)
}

pub fn serialize_receipt_v0(receipt: &ReceiptV0) -> Result<Vec<u8>, ReceiptError> {
    receipt.validate_well_formed()?;

    let mut out = Vec::with_capacity(2 + receipt.vm_id.len() + 32 * 6 + 8 + 8 + 4);

    write_u16_be(&mut out, receipt.bitwasm_version);
    write_string_u16_be(&mut out, &receipt.vm_id)?;
    out.extend_from_slice(&receipt.wasm_hash);
    out.extend_from_slice(&receipt.state_root_before);
    out.extend_from_slice(&receipt.input_hash);
    out.extend_from_slice(&receipt.state_root_after);
    out.extend_from_slice(&receipt.outputs_hash);
    write_u64_be(&mut out, receipt.gas_used);
    write_u64_be(&mut out, receipt.fee_sats);
    write_u32_be(&mut out, receipt.expiry_height);
    out.extend_from_slice(&receipt.federation_id);

    Ok(out)
}

fn write_string_u16_be(out: &mut Vec<u8>, s: &str) -> Result<(), ReceiptError> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len > u16::MAX as usize {
        return Err(ReceiptError::VmIdTooLong { len });
    }
    write_u16_be(out, len as u16);
    out.extend_from_slice(bytes);
    Ok(())
}

fn write_u16_be(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn write_u32_be(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn write_u64_be(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    fn h(hexstr: &str) -> [u8; 32] {
        let bytes = decode(hexstr).expect("hex decode");
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        out
    }

    #[test]
    fn receipt_serialization_is_big_endian() {
        let receipt = ReceiptV0 {
            bitwasm_version: 0x0102,
            vm_id: "wasm32-bitwasm-v0".to_string(),
            wasm_hash: [0x11; 32],
            state_root_before: [0x22; 32],
            input_hash: [0x33; 32],
            state_root_after: [0x44; 32],
            outputs_hash: [0x55; 32],
            gas_used: 0x0102030405060708,
            fee_sats: 0x0a0b0c0d0e0f1011,
            expiry_height: 0x0a0b0c0d,
            federation_id: [0x66; 32],
        };

        let bytes = receipt.canonical_bytes().expect("serialize");

        assert_eq!(&bytes[0..2], &[0x01, 0x02]);

        let vm_len = u16::from_be_bytes([bytes[2], bytes[3]]) as usize;
        assert_eq!(vm_len, receipt.vm_id.len());

        let gas_offset = 2 + 2 + receipt.vm_id.len() + 32 * 6;
        assert_eq!(
            &bytes[gas_offset..gas_offset + 8],
            &receipt.gas_used.to_be_bytes()
        );

        let fee_offset = gas_offset + 8;
        assert_eq!(
            &bytes[fee_offset..fee_offset + 8],
            &receipt.fee_sats.to_be_bytes()
        );

        let expiry_offset = fee_offset + 8;
        assert_eq!(
            &bytes[expiry_offset..expiry_offset + 4],
            &receipt.expiry_height.to_be_bytes()
        );
    }

    #[test]
    fn receipt_hash_matches_expected() {
        let receipt = ReceiptV0 {
            bitwasm_version: 0,
            vm_id: "wasm32-bitwasm-v0".to_string(),
            wasm_hash: h("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"),
            state_root_before: h("202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f"),
            input_hash: h("404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f"),
            state_root_after: h("606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f"),
            outputs_hash: h("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f"),
            gas_used: 1,
            fee_sats: 2,
            expiry_height: 3,
            federation_id: h("a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf"),
        };

        let hash = receipt.hash().expect("hash");
        let expected = h("d1eb74836aa7363284942639b18037a9573e915edc0349106d03af84ae6d27d4");
        assert_eq!(hash, expected);
    }

    #[test]
    fn outputs_hash_is_sha256() {
        let outputs = OutputsV0::new(b"hello".to_vec());
        let expected = h("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        assert_eq!(outputs.hash(), expected);
    }
}
