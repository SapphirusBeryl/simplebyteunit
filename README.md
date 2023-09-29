[![Rust](https://github.com/SapphirusBeryl/simplebyteunit/actions/workflows/rust.yml/badge.svg)](https://github.com/SapphirusBeryl/simplebyteunit/actions/workflows/rust.yml)

# SimpleByteUnit

SimpleByteUnit is a crate which provides a thin encapsulate for integer primitives to facilitate a fast, simple, yet ergonomic byteunit implementation.

## Usage

Add 'simplebyteunit' to your 'Cargo.toml':

```
[dependencies]
simplebyteunit = "0.2.0"
```

## Example

Generate a human-readable, formatted ByteUnit: 

```
use simplebyteunit::simplebyteunit::*;

let byteunit_var = 500000.to_byteunit(SI);

println!("{byteunit_var}");
```

Output:

```
500 kB
```
