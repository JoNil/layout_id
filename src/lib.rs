#![feature(core_intrinsics)]
#![feature(plugin)]

#![cfg_attr(feature = "use_clippy", plugin(clippy))]
#![plugin(regex_macros)]

#![cfg_attr(feature = "use_clippy",
   warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss,
        non_ascii_literal, shadow_same, string_add, string_add_assign, unicode_not_nfc))]

extern crate regex;
extern crate twox_hash;

use regex::Captures;
use std::hash::{Hash, Hasher};
use std::intrinsics;
use twox_hash::XxHash;

pub fn layout_id<T>() -> u64 {

    let mut s = XxHash::with_seed(0);
    
    get_type_name(unsafe { intrinsics::type_name::<T>() }).hash(&mut s);
    unsafe { intrinsics::min_align_of::<T>() }.hash(&mut s);
    unsafe { intrinsics::pref_align_of::<T>() }.hash(&mut s);
    unsafe { intrinsics::size_of::<T>() }.hash(&mut s);

    s.finish()
}

fn get_type_name(name: &str) -> String {

    let inner = regex!(r#"<(.*)>"#);
    let last_name = regex!(r#"[::]?([^:]*)$"#);

    let replaced_children = inner.replace_all(&name, |caps: &Captures| {
        format!("<{}>", get_type_name(&caps.at(1).unwrap()))
    });

    let caps = last_name.captures(&replaced_children).unwrap();

    String::from(caps.at(1).unwrap())
}

#[test]
fn test_get_type_name() {
    assert_eq!(&get_type_name("persistent_hashmap::HashmapEntry<user::User>"),
              "HashmapEntry<User>");
    assert_eq!(&get_type_name("persistent_hashmap::HashmapEntry<meus_iot::user::User>"),
               "HashmapEntry<User>");
    assert_eq!(&get_type_name("persistent_hashmap::HashmapEntry<meus_iot::user::User<test::Type>>"),
               "HashmapEntry<User<Type>>");
    assert_eq!(&get_type_name("HashmapEntry<User>"),
               "HashmapEntry<User>");
}