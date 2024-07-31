#[inline]
#[cold]
fn cold() {}

#[inline(always)]
#[rustfmt::skip]
pub fn likely(b: bool) -> bool {
    if !b { cold() } b
}

#[inline(always)]
#[rustfmt::skip]
pub fn unlikely(b: bool) -> bool {
    if b { cold() } b
}
