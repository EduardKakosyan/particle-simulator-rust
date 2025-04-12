use anyhow::{bail, Error, Result};
use base64::engine::general_purpose;
use base64::Engine as _;
use hash::merhash::mersenne_hash;

/// crypto(length=100), you can exchange/add/delete characters
const CRYPTO: &str = "!pqHr$*+ST1Vst_uv:?wWS%X&Y-/Z01_2.34<ABL9ECo|x#yDE^F{GHEI[]JK>LM#NOBWPQ:Raku@}cde56R7=8f/9gIhi,jkzmn";

/// # Example
/// use encryptor:: password::generate_password;
/// let seed = "jdwnp"
/// let length = 16;
/// let passwd = generate_password(seed, length);
/// match passwd {
///     Ok(val) => println!({"#?"}, val),
///     Err(err) => println!({:#?}, err),
/// }
pub fn generate_password(seed: &str, length: usize)
    -> Result<String, Error>
{
    if length < 6 {
        bail!("length must >= 6");
    }

    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        _ => 3, 
    };

    let mut mer_hash = mersenne_hash(seed).pow(p);
    
    let mut passwd = String::new();
    let crypto_len = CRYPTO.len();
    while mer_hash > 9 {
        let loc = mer_hash % crypto_len;
        let nthc = CRYPTO.chars().nth(loc).expect("Error while getting char!");

        passwd.push(nthc);
        mer_hash /= crypto_len;
    }

    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }

    passwd = general_purpose::STANDARD.encode(passwd);
    passwd = passwd.replace("+", "*").replace("/", "*");

    let interval = passwd.clone();
    while passwd.len() < length {
        passwd += &interval;
    }

    Ok(format!("{}: {}", seed, &passwd[..length]))
}
