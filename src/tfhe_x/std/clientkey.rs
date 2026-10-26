use tfhe::shortint::{ClientKey, parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS};
use super::super::types::Ciphertext;

pub const HEADROOM: u64 = 4; //this is a logical limitation

pub struct StdClientKey
{
    pub(super) inner: ClientKey,
}

impl StdClientKey
{
    pub fn new() -> Self
    {
        StdClientKey
        {
            inner: ClientKey::new(PARAM_MESSAGE_2_CARRY_2_KS_PBS),
        }
    }

    pub fn headroom(&self) -> u64
    {
        HEADROOM
    }

    pub fn encrypt(&mut self, msg: bool) -> Ciphertext
    {
        self.inner.encrypt(msg as u64)
    }

    pub fn decrypt_small(&self, ct: &Ciphertext) -> u64
    {
        self.inner.decrypt_message_and_carry(ct)
    }

    pub fn decrypt(&self, ct: &Ciphertext) -> bool
    {
        match self.decrypt_small(ct)
        {
            0 => false,
            1 => true,
            _ => panic!("invalid decoding"),
        }
    }
}