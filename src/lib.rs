#![allow(dead_code)]
#![allow(non_snake_case)]
#![no_std]
const FILE: &[u8] = include_bytes!(env!(
    "FILE",
    "Missing FILE argument.\n\nUsage:\n\n\tFILE=/etc/passwd cargo build\n"
));
const PADDING: usize = 16 - FILE.len() % 16;
const fn VIEWER() -> ([u8; FILE.len()], [u8; PADDING]) {
    let mut a: [u8; FILE.len()] = [0; FILE.len()];
    let mut i = 0;
    while i < FILE.len() {
        a[i] = FILE[i];
        i += 1;
    }
    (a, unsafe { core::mem::MaybeUninit::uninit().assume_init() })
}
type HEX = ([u8; FILE.len()], [u8; PADDING]);
const RUSTC: HEX = VIEWER();
