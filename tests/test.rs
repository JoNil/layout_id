// Copyright (c) 2016 Jonathan Nilsson
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate layout_id;

use layout_id::layout_id;

#[test]
fn test() {
    assert_eq!(layout_id::<u32>(), 7198607969759593990);
    assert_eq!(layout_id::<u64>(), 12112908602430017274);
    assert_eq!(layout_id::<i64>(), 9754572191070252073);
}