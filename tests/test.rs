extern crate layout_id;

use layout_id::layout_id;

#[test]
fn test() {
    assert_eq!(layout_id::<u32>(), 7198607969759593990);
    assert_eq!(layout_id::<u64>(), 12112908602430017274);
    assert_eq!(layout_id::<i64>(), 9754572191070252073);
}