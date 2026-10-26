use tfhe::shortint::ServerKey;
use tfhe::shortint::server_key::LookupTableOwned;
use super::clientkey::{StdClientKey, HEADROOM};
use super::super::types::Ciphertext;

pub struct StdServerKey
{
    inner: ServerKey,
    and_acc:  LookupTableOwned,
    nand_acc: LookupTableOwned,
    or_acc:   LookupTableOwned,
    nor_acc:  LookupTableOwned,
    xor_acc:  LookupTableOwned,
    xnor_acc: LookupTableOwned,
    not_acc:  LookupTableOwned,
}

impl StdServerKey
{
    pub fn new(ck: &mut StdClientKey) -> Self
    {
        let inner = ServerKey::new(&ck.inner);

        // the input to these tables is the sum of the two bits: 0, 1 or 2
        let and_acc  = inner.generate_lookup_table(|s| (s == 2) as u64);
        let nand_acc = inner.generate_lookup_table(|s| (s != 2) as u64);
        let or_acc   = inner.generate_lookup_table(|s| (s >= 1) as u64);
        let nor_acc  = inner.generate_lookup_table(|s| (s == 0) as u64);
        let xor_acc  = inner.generate_lookup_table(|s| (s == 1) as u64);
        let xnor_acc = inner.generate_lookup_table(|s| (s != 1) as u64);
        let not_acc  = inner.generate_lookup_table(|s| (s == 0) as u64); // 0 -> 1, 1 -> 0

        StdServerKey { inner, and_acc, nand_acc, or_acc, nor_acc, xor_acc, xnor_acc, not_acc }
    }

    pub fn headroom(&self) -> u64
    {
        HEADROOM
    }

    fn bootstrap(&self, input: &Ciphertext, lut: &LookupTableOwned) -> Ciphertext
    {
        // PBS + keyswitch, all handled inside shortint
        self.inner.apply_lookup_table(input, lut)
    }

    /// Adds the two bits (free), then bootstraps the sum through the given table.
    fn gate(&self, ct1: &Ciphertext, ct2: &Ciphertext, lut: &LookupTableOwned) -> Ciphertext
    {
        let sum = self.inner.unchecked_add(ct1, ct2);
        self.bootstrap(&sum, lut)
    }

    pub fn xor(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.gate(ct1, ct2, &self.xor_acc)
    }

    pub fn xnor(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.gate(ct1, ct2, &self.xnor_acc)
    }

    pub fn and(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.gate(ct1, ct2, &self.and_acc)
    }

    pub fn nand(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.gate(ct1, ct2, &self.nand_acc)
    }

    pub fn or(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.gate(ct1, ct2, &self.or_acc)
    }

    pub fn nor(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.gate(ct1, ct2, &self.nor_acc)
    }

    pub fn not(&self, ct: &Ciphertext) -> Ciphertext
    {
        self.bootstrap(ct, &self.not_acc)
    }

    pub fn trivial_encrypt(&self, msg: bool) -> Ciphertext
    {
        self.inner.create_trivial(msg as u64)
    }
}