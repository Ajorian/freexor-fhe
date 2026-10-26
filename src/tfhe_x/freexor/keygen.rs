use super::clientkey::FxClientKey;
use super::serverkey::FxServerKey;

pub fn generate_keys() -> (FxClientKey, FxServerKey)
{
    let mut ck = FxClientKey::new();
    let sk = FxServerKey::new(&mut ck);
    (ck, sk)
}