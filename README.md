# "Writing a C Compiler" implementation in Rust

Implementation of the C Compiler for [Nora Sandler's "Writing a C Compiler"](https://nostarch.com/writing-c-compiler) in Rust.

Used by myself to learn some rust.

# Usage

Compile using cargo

```
cargo build
```

then run the compiler driver on your C file with either --lex, --parse, --codegen or no option:

```
./driver C_SOURCE_FILE.c --lex
```
