/*
 * SimpleByteUnit
 *
 * Copyright (C) 2023-2025 Xavier Moffett <sapphirus@azorium.net>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 */

/*!
A thin encapsulate for integer primitives to facilitate a fast, simple, yet ergonomic byteunit implementation.

# Getting Started

Add 'simplebyteunit' to your 'Cargo.toml':

```toml
[dependencies]
simplebyteunit = "0.3.0"
```

## Example

Generate a human-readable, formatted ByteUnit:

```rust
use simplebyteunit::simplebyteunit::*;

let byteunit_var = 500000.to_byteunit(SI);

assert_eq!(byteunit_var.to_string(), "500.00 kB");
```

## Simple arithmetic operations

Addition, subtraction, multiplication, subtraction, and division are supported on this type.

```rust
use simplebyteunit::simplebyteunit::*;

let a: ByteUnit<i64> = ByteUnit::SI(500000);
let b: ByteUnit<i64> = ByteUnit::SI(500000);

assert_eq!(a + b, "1.0 MB".into());
```

## Equal/and/or operations

Equal operations are supported on this type:

```rust
use simplebyteunit::simplebyteunit::*;

let a: ByteUnit<i64> = "500 KiB".into();
let b: ByteUnit<i64> = "500 KiB".into();

assert_eq!(a == b, true);
```

Or operations are also supported on this type:

```rust
use simplebyteunit::simplebyteunit::*;

let a: ByteUnit<i64> = 5000000.to_byteunit(IEC);
let b: ByteUnit<i64> = 5000000.to_byteunit(IEC);

assert_eq!(a >= b, true);
```
*/

pub mod simplebyteunit;

mod arithmetic;
mod suffix;
