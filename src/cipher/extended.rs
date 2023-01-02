use super::Cipher;

const SIG_BIT_MASK: u8 = 127;
const LOWER_BITS_MASK: u8 = 63;
const SINGLE_CHAR_MASK: u8 = 4;

pub struct Extended {}

impl Cipher<4> for Extended {
    fn encrypt_char_pair(&self, c0: u8, c1: u8) -> [u8; 4] {
        let mut encrypted_char = [0, 0, 0, 0];
        let sig_0 = c0 & SIG_BIT_MASK;
        let sig_1 = c1 & SIG_BIT_MASK;
        let low_0 = c0 & LOWER_BITS_MASK;
        let low_1 = c1 & LOWER_BITS_MASK;
        encrypted_char[0] = 240;
        encrypted_char[1] = 144 | (sig_0 >> 5) | (sig_1 >> 6);
        encrypted_char[2] = 128 | low_0;
        encrypted_char[3] = 128 | low_1;
        return encrypted_char;
    }

    fn encrypt_single_char(&self, c0: u8) -> [u8; 4] {
        let mut encrypted_char = [0, 0, 0, 0];
        let sig_0 = c0 & SIG_BIT_MASK;
        let low_0 = c0 & LOWER_BITS_MASK;
        encrypted_char[0] = 240;
        encrypted_char[1] = 144 | SINGLE_CHAR_MASK | (sig_0 >> 6);
        encrypted_char[2] = 128 | low_0;
        encrypted_char[3] = 128;
        return encrypted_char;
    }

    fn decrypt_char_pair(&self, encrypted: [u8; 4]) -> [u8; 2] {
        let c0_sig_bit = (encrypted[1] & 2) << 5;
        let c1_sig_bit = (encrypted[1] & 1) << 6;
        let c0_lower = encrypted[2] & LOWER_BITS_MASK;
        let c1_lower = encrypted[3] & LOWER_BITS_MASK;
        let c0 = c0_sig_bit | c0_lower;
        let c1 = c1_sig_bit | c1_lower;
        return [c0, c1];
    }

    fn decrypt_single_char(&self, encrypted: [u8; 4]) -> [u8; 1] {
        let sig_bit = (encrypted[1] & 1) << 6;
        let lower = encrypted[2] & LOWER_BITS_MASK;
        return [sig_bit | lower];
    }

    fn has_single_char(&self, encrypted: [u8; 4]) -> bool {
        encrypted[1] & SINGLE_CHAR_MASK != 0
    }
}