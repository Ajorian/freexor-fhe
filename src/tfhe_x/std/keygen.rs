use super::clientkey::StdClientKey;
use super::serverkey::StdServerKey;

pub fn generate_keys() -> (StdClientKey, StdServerKey)
{
    let mut ck = StdClientKey::new();
    let sk = StdServerKey::new(&mut ck);
    (ck, sk)
}