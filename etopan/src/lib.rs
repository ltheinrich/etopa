//! Etopa for Android

#![allow(non_snake_case)]

use etopa::Fail;
use etopa::crypto::argon2_hash;
use etopa::crypto::encrypt;
use etopa::crypto::hash_key;
use etopa::crypto::hash_name;
use etopa::crypto::hash_password;
use etopa::crypto::hash_pin;
use etopa::crypto::hex_decode;
use etopa::crypto::hex_encode;
use etopa::crypto::random;
use etopa::{crypto::decrypt, totp::Generator};
use jni::EnvUnowned;
use jni::Outcome;
use jni::objects::{JObject, JString};
use jni::sys::jstring;

type JniResult<T> = std::result::Result<T, jni::errors::Error>;

/// Receive string from Java
pub fn recv_string(env: &mut EnvUnowned, input: JString) -> String {
    match env
        .with_env(|_| -> JniResult<_> { Ok(input.to_string()) })
        .into_outcome()
    {
        Outcome::Ok(input) => Ok(input),
        Outcome::Err(err) => Fail::from(err),
        Outcome::Panic(err) => Fail::from(format!("{:?}", err)),
    }
    .unwrap() // should not fail
}

/// Make string for Java
pub fn make_string(env: &mut EnvUnowned, string: impl AsRef<str>) -> jstring {
    match env
        .with_env(|env| -> JniResult<_> { env.new_string(string) })
        .into_outcome()
    {
        Outcome::Ok(input) => Ok(input.into_raw()),
        Outcome::Err(err) => Fail::from(err),
        Outcome::Panic(err) => Fail::from(format!("{:?}", err)),
    }
    .unwrap() // should not fail
}

/// Empty string for Java
pub fn empty_string(env: &mut EnvUnowned) -> jstring {
    make_string(env, "")
}

/// Hash key
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashKey(
    mut env: EnvUnowned,
    _: JObject,
    jkey: JString,
) -> jstring {
    // receive and hash key
    let key = recv_string(&mut env, jkey);
    make_string(&mut env, hash_key(key))
}

/// Hash password
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashPassword(
    mut env: EnvUnowned,
    _: JObject,
    jpassword: JString,
) -> jstring {
    // receive and hash password
    let password = recv_string(&mut env, jpassword);
    make_string(&mut env, hash_password(password))
}

/// Hash pin
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashPin(
    mut env: EnvUnowned,
    _: JObject,
    jpin: JString,
) -> jstring {
    // receive and hash pin
    let pin = recv_string(&mut env, jpin);
    make_string(&mut env, hash_pin(pin))
}

/// Hash name
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashName(
    mut env: EnvUnowned,
    _: JObject,
    jname: JString,
) -> jstring {
    // receive and hash secret name
    let name = recv_string(&mut env, jname);
    make_string(&mut env, hash_name(name))
}

/// Hash hashed password using Argon2
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashArgon2Hashed(
    mut env: EnvUnowned,
    _: JObject,
    jpassword_hash: JString,
) -> jstring {
    // receive password
    let password_hash = recv_string(&mut env, jpassword_hash);

    // generate salt and hash password
    let salt = random(16);

    // generate and return argon2 hash
    make_string(&mut env, argon2_hash(password_hash, salt).unwrap())
}

/// Encrypt
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_encrypt(
    mut env: EnvUnowned,
    _: JObject,
    jkey: JString,
    jdata: JString,
) -> jstring {
    let key = recv_string(&mut env, jkey);
    let data = recv_string(&mut env, jdata);

    // encrypt
    let encrypted = encrypt(data, key);

    // encode
    let encoded = match encrypted {
        Ok(encrypted) => hex_encode(encrypted),
        _ => String::new(),
    };

    make_string(&mut env, encoded)
}

/// Decrypt
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_decrypt(
    mut env: EnvUnowned,
    _: JObject,
    jkey: JString,
    jdata: JString,
) -> jstring {
    let key = recv_string(&mut env, jkey);
    let data = recv_string(&mut env, jdata);

    // decode
    let decoded = match hex_decode(data) {
        Ok(decoded) => decoded,
        _ => return empty_string(&mut env),
    };

    // decrypt
    let decrypted = decrypt(decoded, key).unwrap_or_default();

    make_string(&mut env, decrypted)
}

/// Generate token
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_generateToken(
    mut env: EnvUnowned,
    _: JObject,
    jsecret: JString,
) -> jstring {
    // receive secret
    let secret = recv_string(&mut env, jsecret);

    // create token generator
    let token = match Generator::new(secret) {
        // generate token
        Ok(r#gen) => r#gen.token().unwrap_or_else(|_| "invalid".to_string()),
        _ => "invalid".to_string(),
    };

    // return token
    make_string(&mut env, token)
}

/// Decode URL
#[unsafe(no_mangle)]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_decodeUrl(
    mut env: EnvUnowned,
    _: JObject,
    jencoded_url: JString,
) -> jstring {
    // receive encoded url
    let encoded_url = recv_string(&mut env, jencoded_url);

    let url = encoded_url
        .replace("%20", " ")
        .replace("%21", "!")
        .replace("%22", "\"")
        .replace("%23", "#")
        .replace("%24", "$")
        .replace("%25", "%")
        .replace("%26", "&")
        .replace("%27", "'")
        .replace("%28", "(")
        .replace("%29", ")")
        .replace("%2A", "*")
        .replace("%2B", "+")
        .replace("%2C", ",")
        .replace("%2D", "-")
        .replace("%2E", ".")
        .replace("%2F", "/")
        .replace("%3A", ":")
        .replace("%3B", ";")
        .replace("%3C", "<")
        .replace("%3D", "=")
        .replace("%3E", ">")
        .replace("%3F", "?")
        .replace("%40", "@")
        .replace("%5B", "[")
        .replace("%5C", "\\")
        .replace("%5D", "]")
        .replace("%7B", "{")
        .replace("%7C", "|")
        .replace("%7D", "}");

    // return token
    make_string(&mut env, url)
}
