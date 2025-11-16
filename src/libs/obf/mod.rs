use crate::libs::graphite::GraphiteMetric;
use ahash::AHasher;
use std::hash::{Hash, Hasher};

// maximum resulted metric length
// name + 16 tags * 32 bytes
pub const MAX_METRIC_LEN: usize = 512;

#[inline(always)]
fn fast_hash(input: &str) -> u64 {
    let mut h = AHasher::default();
    input.hash(&mut h);
    h.finish()
}

// write u64 hex directly into buffer
#[inline(always)]
fn write_hex(mut n: u64, buf: &mut [u8], pos: &mut usize) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    assert!(*pos + 16 <= buf.len(), "buffer overflow in write_hex");
    for i in (0..16).rev() {
        buf[*pos + i] = HEX[(n & 0xF) as usize];
        n >>= 4;
    }
    *pos += 16;
}

// obfuscate one metric and write into buffer
pub fn obfuscate<'a>(metric: &GraphiteMetric, buf: &'a mut [u8; MAX_METRIC_LEN]) -> &'a str {
    let mut pos = 0;

    // name
    assert!(pos + 4 + 16 <= MAX_METRIC_LEN, "buffer overflow at name");
    buf[pos..pos + 4].copy_from_slice(b"obf_");
    pos += 4;
    write_hex(fast_hash(metric.name), buf, &mut pos);

    // tags
    for (key, value) in &metric.tags {
        assert!(
            pos + 1 + key.len() + 1 + 4 + 16 <= MAX_METRIC_LEN,
            "buffer overflow at tag: key='{}', pos={}",
            key,
            pos
        );
        buf[pos] = b';';
        pos += 1;
        buf[pos..pos + key.len()].copy_from_slice(key.as_bytes());
        pos += key.len();
        buf[pos] = b'=';
        pos += 1;
        buf[pos..pos + 4].copy_from_slice(b"obf_");
        pos += 4;
        write_hex(fast_hash(value), buf, &mut pos);
    }

    // safe transform data from buffer to &str
    std::str::from_utf8(&buf[..pos]).unwrap()
}
