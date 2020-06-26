//! Etopa for Android

#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use etopa::crypto::hash_key;
use etopa::crypto::hash_password;
use etopa::crypto::hash_pin;
use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::{CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn Java_de_ltheinrich_etopa_utils_NativeKt_hashKey(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient = CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ));

    let output = env
        .new_string(hash_key(recipient.to_str().unwrap()))
        .unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_ltheinrich_etopa_utils_NativeKt_hashPassword(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient = CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ));

    let output = env
        .new_string(hash_password(recipient.to_str().unwrap()))
        .unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_ltheinrich_etopa_utils_NativeKt_hashPin(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient = CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ));

    let output = env
        .new_string(hash_pin(recipient.to_str().unwrap()))
        .unwrap();
    output.into_inner()
}
