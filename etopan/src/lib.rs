//! Etopa for Android

#![allow(non_snake_case)]

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
use jni::objects::{JObject, JString};
use jni::strings::JNIString;
use jni::sys::jstring;
use jni::JNIEnv;

/// Receive string from Java
pub fn recv_string(env: &JNIEnv, input: JString) -> String {
    env.get_string(input).unwrap().into()
}

/// Make string for Java
pub fn make_string(env: &JNIEnv, string: impl Into<JNIString>) -> jstring {
    let output = env.new_string(string).unwrap();
    output.into_inner()
}

/// Empty string for Java
pub fn empty_string(env: &JNIEnv) -> jstring {
    make_string(env, "")
}

/// Hash key
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashKey(
    env: JNIEnv,
    _: JObject,
    jkey: JString,
) -> jstring {
    // receive and hash key
    let key = recv_string(&env, jkey);
    make_string(&env, hash_key(key))
}

/// Hash password
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashPassword(
    env: JNIEnv,
    _: JObject,
    jpassword: JString,
) -> jstring {
    // receive and hash password
    let password = recv_string(&env, jpassword);
    make_string(&env, hash_password(password))
}

/// Hash pin
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashPin(
    env: JNIEnv,
    _: JObject,
    jpin: JString,
) -> jstring {
    // receive and hash pin
    let pin = recv_string(&env, jpin);
    make_string(&env, hash_pin(pin))
}

/// Hash name
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashName(
    env: JNIEnv,
    _: JObject,
    jname: JString,
) -> jstring {
    // receive and hash secret name
    let name = recv_string(&env, jname);
    make_string(&env, hash_name(name))
}

/// Hash hashed password using Argon2
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_hashArgon2Hashed(
    env: JNIEnv,
    _: JObject,
    jpassword_hash: JString,
) -> jstring {
    // receive password
    let password_hash = recv_string(&env, jpassword_hash);

    // generate salt and hash password
    let salt = random(16);

    // generate and return argon2 hash
    make_string(&env, argon2_hash(password_hash, salt).unwrap())
}

/// Encrypt
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_encrypt(
    env: JNIEnv,
    _: JObject,
    jkey: JString,
    jdata: JString,
) -> jstring {
    let key = recv_string(&env, jkey);
    let data = recv_string(&env, jdata);

    // encrypt
    let encrypted = encrypt(data, key);

    // encode
    let encoded = match encrypted {
        Ok(encrypted) => hex_encode(encrypted),
        _ => String::new(),
    };

    make_string(&env, encoded)
}

/// Decrypt
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_decrypt(
    env: JNIEnv,
    _: JObject,
    jkey: JString,
    jdata: JString,
) -> jstring {
    let key = recv_string(&env, jkey);
    let data = recv_string(&env, jdata);

    // decode
    let decoded = match hex_decode(data) {
        Ok(decoded) => decoded,
        _ => return empty_string(&env),
    };

    // decrypt
    let decrypted = match decrypt(decoded, key) {
        Ok(decrypted) => decrypted,
        _ => String::new(),
    };

    make_string(&env, decrypted)
}

/// Generate token
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_generateToken(
    env: JNIEnv,
    _: JObject,
    jsecret: JString,
) -> jstring {
    // receive secret
    let secret = recv_string(&env, jsecret);

    // create token generator
    let token = match Generator::new(secret) {
        // generate token
        Ok(gen) => gen.token().unwrap_or_else(|_| "invalid".to_string()),
        _ => "invalid".to_string(),
    };

    // return token
    make_string(&env, token)
}

/// Decode URL
#[no_mangle]
pub extern "C" fn Java_de_ltheinrich_etopa_utils_Common_decodeUrl(
    env: JNIEnv,
    _: JObject,
    jencoded_url: JString,
) -> jstring {
    // receive encoded url
    let encoded_url = recv_string(&env, jencoded_url);

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
    make_string(&env, url)
}
