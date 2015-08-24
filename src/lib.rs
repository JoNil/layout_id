#![feature(core_intrinsics)]

extern crate twox_hash;

use std::hash::{Hash, Hasher};
use std::intrinsics;
use twox_hash::XxHash;

pub fn layout_id<T>() -> u64 {

    let mut s = XxHash::with_seed(0);
    
    unsafe { intrinsics::type_name::<T>() }.hash(&mut s);
    unsafe { intrinsics::min_align_of::<T>() }.hash(&mut s);
    unsafe { intrinsics::pref_align_of::<T>() }.hash(&mut s);
    unsafe { intrinsics::size_of::<T>() }.hash(&mut s);

    s.finish()
}