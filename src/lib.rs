use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // Attempt to decode the hexadecimal string into a byte vector.
    // If decoding fails, map the error to a String and return it.
    decode(hex_str).map_err(|e| e.to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // Reverse the order of the bytes in the input slice and collect them into a new Vec<u8>.
    bytes.iter().rev().cloned().collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // Encode the byte slice into a hexadecimal string.
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // Decode the hexadecimal string into a byte vector. The `hex::decode` function
    // already returns a `Result<Vec<u8>, hex::FromHexError>`, so we can just return its output.
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // Convert the u32 into its byte representation in little-endian order.
    // `to_le_bytes()` performs the little-endian conversion.
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // Attempt to parse the input string into a u64.
    // If parsing succeeds, return Ok(value).
    // If parsing fails (e.g., input is not a valid number), return an error string.
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

#[derive(Debug, PartialEq)] // Derive Debug and PartialEq for easy printing and comparison in tests.
pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // Check if the script starts with the P2PKH pattern (OP_DUP OP_HASH160 OP_PUSHBYTES_20).
    // 0x76 is OP_DUP, 0xa9 is OP_HASH160, 0x14 is PUSHBYTES_20.
    if script.starts_with(&[0x76, 0xa9, 0x14]) {
        ScriptType::P2PKH
    // Check if the script starts with the P2WPKH pattern (OP_0 OP_PUSHBYTES_20).
    // 0x00 is OP_0, 0x14 is PUSHBYTES_20.
    } else if script.starts_with(&[0x00, 0x14]) {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown
    }
}

// Outpoint tuple struct with a String for txid and u32 for vout.
// I need to add necessary derive traits for cloning, equality, and debugging.
#[derive(Debug, Clone, PartialEq)]
pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // Return the slice of the script starting from index 2 to the end.
    // This assumes the pushdata starts at the third byte (index 2).
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // Return the confirmed balance of the wallet.
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // Subtract the fee from the mutable balance reference.
    *balance = balance.saturating_sub(fee); // Use saturating_sub to prevent underflow if fee > balance.
}

pub fn move_txid(txid: String) -> String {
    // Format the txid string for display or logging.
    format!("txid: {}", txid)
}

// Add necessary derive traits for equality, debugging, and cloning.
#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // Match the input byte to the corresponding Opcode variant.
        // Return an error string if the byte does not match any known opcode.
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

// Add necessary derive traits for debugging, cloning, and equality.
#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // In this simple case, "consuming" the UTXO just means returning it.
    // In a real application, this might involve removing it from a UTXO set
    // or marking it as spent. For the purpose of this exercise, we just return it.
    utxo
}
