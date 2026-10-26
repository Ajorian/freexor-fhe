use tfhe::shortint::ServerKey;
use tfhe::shortint::server_key::LookupTableOwned;
use super::clientkey::{FxClientKey, HEADROOM};
use super::super::types::Ciphertext;

pub struct FxServerKey
{
    inner: ServerKey,
    pi_acc:   LookupTableOwned,
    and_acc:  LookupTableOwned,
    nand_acc: LookupTableOwned,
    or_acc:   LookupTableOwned,
    nor_acc:  LookupTableOwned,
}

impl FxServerKey
{
    pub fn new(ck: &mut FxClientKey) -> Self
    {
        let inner = ServerKey::new(&ck.inner);

        // the input to these tables is the "index" 2*b0 + parity(b1)
        let pi_acc   = inner.generate_lookup_table(|x| x % 2);
        let and_acc  = inner.generate_lookup_table(|x| (x % 4 == 3) as u64);
        let nand_acc = inner.generate_lookup_table(|x| (x % 4 != 3) as u64);
        let or_acc   = inner.generate_lookup_table(|x| (x % 4 != 0) as u64);
        let nor_acc  = inner.generate_lookup_table(|x| (x % 4 == 0) as u64);

        FxServerKey { inner, pi_acc, and_acc, nand_acc, or_acc, nor_acc }
    }

    pub fn headroom(&self) -> u32
    {
        HEADROOM
    }

    fn bootstrap(&self, input: &Ciphertext, lut: &LookupTableOwned) -> Ciphertext
    {
        // PBS + keyswitch, all handled inside shortint
        self.inner.apply_lookup_table(input, lut)
    }

    fn get_index(&self, ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext
    {
        let twice_ct0 = self.inner.unchecked_scalar_mul(ct0, 2); // 2*ct0
        let pied_c1 = self.bootstrap(ct1, &self.pi_acc);         // parity(ct1)
        self.inner.unchecked_add(&twice_ct0, &pied_c1)           // 2*ct0 + parity(ct1)
    }

    pub fn xor(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        self.inner.unchecked_add(ct1, ct2) // free, no bootstrap
    }

    pub fn xnor(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext
    {
        let x = self.xor(ct1, ct2);
        self.inner.unchecked_scalar_add(&x, 1)
    }

    pub fn and(&self, ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext
    {
        let i = self.get_index(ct0, ct1);
        self.bootstrap(&i, &self.and_acc)
    }

    pub fn nand(&self, ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext
    {
        let i = self.get_index(ct0, ct1);
        self.bootstrap(&i, &self.nand_acc)
    }

    pub fn or(&self, ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext
    {
        let i = self.get_index(ct0, ct1);
        self.bootstrap(&i, &self.or_acc)
    }

    pub fn nor(&self, ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext
    {
        let i = self.get_index(ct0, ct1);
        self.bootstrap(&i, &self.nor_acc)
    }

    pub fn refresh(&self, ct: &Ciphertext) -> Ciphertext
    {
        self.bootstrap(ct, &self.pi_acc)
    }

    pub fn not(&self, ct: &Ciphertext) -> Ciphertext
    {
        self.inner.unchecked_scalar_add(ct, 1)
    }

    pub fn trivial_encrypt(&self, msg: bool) -> Ciphertext
    {
        self.inner.create_trivial(msg as u64)
    }
}