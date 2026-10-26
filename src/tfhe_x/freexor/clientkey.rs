use tfhe::shortint::{ClientKey, parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS};
use super::super::types::Ciphertext;

/// message (2 bits) + carry (2 bits) = 16 usable values
pub const HEADROOM: u32 = 16;

pub struct FxClientKey
{
    pub(super) inner: ClientKey,
}

impl FxClientKey
{
    pub fn new() -> Self
    {
        FxClientKey
        {
            inner: ClientKey::new(PARAM_MESSAGE_2_CARRY_2_KS_PBS),
        }
    }

    pub fn headroom(&self) -> u32
    {
        HEADROOM
    }

    pub fn encrypt_small(&mut self, m: u64) -> Ciphertext
    {
        self.inner.unchecked_encrypt(m)
    }

    pub fn encrypt(&mut self, msg: bool) -> Ciphertext
    {
        self.encrypt_small(msg as u64)
    }

    pub fn decrypt_small(&self, ct: &Ciphertext) -> u64
    {
        self.inner.decrypt_message_and_carry(ct)
    }

    pub fn decrypt(&self, ct: &Ciphertext) -> bool
    {
        match self.decrypt_small(ct) % 4
        {
            0 | 2 => false,
            1 | 3 => true,
            _ => panic!("invalid decoding"),
        }
    }
}