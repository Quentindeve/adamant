use libadamant::mem::{align_down, align_up, PAGE_SIZE};

#[test]
fn align_up_works_when_align_needed() {
    let addr = 8193;
    assert_eq!(align_up(addr, PAGE_SIZE), 8192 + 4096);
}

#[test]
fn align_down_works_when_align_needed() {
    let addr = 8193;
    assert_eq!(align_down(addr, PAGE_SIZE), 8192);
}

#[test]
fn align_up_works_when_align_not_needed() {
    assert_eq!(align_up(8192, PAGE_SIZE), 8192);
}

#[test]
fn align_down_works_when_align_not_needed() {
    assert_eq!(align_down(8192, PAGE_SIZE), 8192);
}
