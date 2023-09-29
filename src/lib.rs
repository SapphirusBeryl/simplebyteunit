/*  
 * SimpleByteUnit
 * 
 * Copyright 2023 Xaver R.M.
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
simplebyteunit = "0.2.0"
```

## Example

Generate a human-readable, formatted ByteUnit: 


```rust
use simplebyteunit::simplebyteunit::*;

let byteunit_var = 500000.to_byteunit(SI);

println!("{byteunit_var}");

```

Output:

```shell
500 kB
````

## Parsing strings into ByteUnits

And then you can parse formatted strings back into a ByteUnit

```rust
use simplebyteunit::simplebyteunit::*;

let byteunit_var: ByteUnit<i64> = "500 kB".into();

println!("{byteunit_var}");

```

Output:

```shell
500 kB
````

## Simple arithmetic operations

Addition, subtraction, multiplication, subtraction, and division are supported on this type.

```rust
use simplebyteunit::simplebyteunit::*;

let a: ByteUnit<i64> = ByteUnit::SI(5000000);
let b: ByteUnit<i64> = ByteUnit::SI(5000000);
let byteunit_sum = a + b;

println!("{byteunit_sum}");

```

Output:

```shell
1.0 MB
```

## Equal/and/or operations

Equal operations are supported on this type:

```rust
use simplebyteunit::simplebyteunit::*;

let a: ByteUnit<i64> = "500 KiB".into();
let b: ByteUnit<i64> = "500 KiB".into();
let byteunit_bool = a == b;

println!("{byteunit_bool}");

```

Output:
```shell
true
````

Or operations are also supported on this type:

```rust
use simplebyteunit::simplebyteunit::*;

let a: ByteUnit<i64> = 5000000.to_byteunit(IEC);
let b: ByteUnit<i64> = 5000000.to_byteunit(IEC);
let byteunit_bool = a >= b;

println!("{byteunit_bool}");

```

Output:
```shell
true
```

*/

pub mod simplebyteunit;

mod input;
mod output;
mod test;
