# rustc-hex-viewer

Did you know that the Rust compiler has a built-in hexadecimal viewer?

This crate implements a simple interface to access that feature, just compile it with the file you want to read in the FILE enviroment variable:

```
git clone https://github.com/Badel2/rustc-hex-viewer
cd rustc-hex-viewer
FILE=/etc/passwd cargo build
```

### Example output:

```
$ FILE=<(head -c 31 /dev/urandom) cargo build
   Compiling rustc-hex-viewer v0.1.0 (/home/bdl/projects/rustc-hex-viewer)
error[E0080]: it is undefined behavior to use this value
  --> src/lib.rs:21:1
   |
21 | const RUSTC: HEX = VIEWER();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .1[31]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 32, align: 1) {
               0x00 │ 90 5a 59 34 62 5d 3a e9 15 95 74 cf c4 4e 3b 08 │ .ZY4b]:...t..N;.
               0x10 │ c7 e3 41 83 57 b6 eb 92 40 39 d8 61 a0 9b 07 __ │ ..A.W...@9.a...░
           }

For more information about this error, try `rustc --explain E0080`.
error: could not compile `rustc-hex-viewer` due to previous error
```
