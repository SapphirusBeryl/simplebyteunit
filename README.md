# SimpleByteUnit [![Build Workflow](https://git.sapphirus.org/Sapphirus/SimpleByteUnit/badges/workflows/check.yml/badge.svg?label=build&logo=github+actions&logoColor=d1d7e0&style=flat-square)](https://git.sapphirus.org/Sapphirus/SimpleByteUnit/actions?workflow=check.yml)

SimpleByteUnit is a crate which provides a thin encapsulate for integer primitives to facilitate a fast, simple, yet ergonomic byteunit implementation.

## Usage

Add 'simplebyteunit' to your 'Cargo.toml':

```
[dependencies]
simplebyteunit = "0.3.0"
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
