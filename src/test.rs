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

#![allow(dead_code,unused_imports)]

use super::simplebyteunit::*;

const POSITIVE_5B: i64 = 5000;
const NEGATIVE_5B: i64  = -5000;
const POSITIVE_5K: i64  = 5000000;
const NEGATIVE_5K: i64 = -5000000;
const POSITIVE_5G: i64 = 5000000000;
const NEGATIVE_5G: i64 = -5000000000;
const POSITIVE_5T: i64 = 5000000000000;
const NEGATIVE_5T: i64 = -5000000000000;
const POSITIVE_5P: i64 = 5000000000000000;
const NEGATIVE_5P: i64 = -5000000000000000;
const POSITIVE_5E: i64 = 5000000000000000000;
const NEGATIVE_5E: i64 = -5000000000000000000;

#[test]
fn format_all() {
    assert_eq!(NEGATIVE_5E.to_byteunit(SI).to_string(), "-5.00 EB");   
    assert_eq!(POSITIVE_5E.to_byteunit(SI).to_string(), "5.00 EB");  
    assert_eq!(NEGATIVE_5P.to_byteunit(SI).to_string(), "-5.00 PB");   
    assert_eq!(POSITIVE_5P.to_byteunit(SI).to_string(), "5.00 PB");  
    assert_eq!(NEGATIVE_5T.to_byteunit(SI).to_string(), "-5.00 TB");  
    assert_eq!(POSITIVE_5T.to_byteunit(SI).to_string(), "5.00 TB");  
    assert_eq!(NEGATIVE_5G.to_byteunit(SI).to_string(), "-5.00 GB");  
    assert_eq!(POSITIVE_5G.to_byteunit(SI).to_string(), "5.00 GB");  
    assert_eq!(POSITIVE_5K.to_byteunit(SI).to_string(), "5.00 MB"); 
    assert_eq!(NEGATIVE_5K.to_byteunit(SI).to_string(), "-5.00 MB"); 
    assert_eq!(POSITIVE_5B.to_byteunit(SI).to_string(), "5.00 kB");
    assert_eq!(NEGATIVE_5B.to_byteunit(SI).to_string(), "-5.00 kB");
    assert_eq!(NEGATIVE_5E.to_byteunit(IEC).to_string(), "-4.34 EiB");   
    assert_eq!(POSITIVE_5E.to_byteunit(IEC).to_string(), "4.34 EiB");  
    assert_eq!(NEGATIVE_5P.to_byteunit(IEC).to_string(), "-4.44 PiB");   
    assert_eq!(POSITIVE_5P.to_byteunit(IEC).to_string(), "4.44 PiB");  
    assert_eq!(NEGATIVE_5T.to_byteunit(IEC).to_string(), "-4.55 TiB");
    assert_eq!(NEGATIVE_5T.to_byteunit(IEC).to_string(), "-4.55 TiB");  
    assert_eq!(POSITIVE_5T.to_byteunit(IEC).to_string(), "4.55 TiB");  
    assert_eq!(NEGATIVE_5G.to_byteunit(IEC).to_string(), "-4.66 GiB");  
    assert_eq!(POSITIVE_5G.to_byteunit(IEC).to_string(), "4.66 GiB");  
    assert_eq!(POSITIVE_5K.to_byteunit(IEC).to_string(), "4.77 MiB"); 
    assert_eq!(NEGATIVE_5K.to_byteunit(IEC).to_string(), "-4.77 MiB"); 
    assert_eq!(POSITIVE_5B.to_byteunit(IEC).to_string(), "4.88 KiB");
    assert_eq!(NEGATIVE_5B.to_byteunit(IEC).to_string(), "-4.88 KiB");
 
}

#[test]
fn bytes() {
    assert_eq!(NEGATIVE_5B.to_byteunit(IEC).pow(B), "-5000 B");   
    assert_eq!(NEGATIVE_5B.to_byteunit(SI).pow(B), "-5000 B");   
    assert_eq!(POSITIVE_5B.to_byteunit(IEC).pow(B), "5000 B");   
    assert_eq!(POSITIVE_5B.to_byteunit(SI).pow(B), "5000 B");     
    assert_eq!(NEGATIVE_5E.to_byteunit(IEC).pow(B), "-5000000000000000000 B");   
    assert_eq!(NEGATIVE_5E.to_byteunit(SI).pow(B), "-5000000000000000000 B");   
    assert_eq!(POSITIVE_5E.to_byteunit(IEC).pow(B), "5000000000000000000 B");   
    assert_eq!(POSITIVE_5E.to_byteunit(SI).pow(B), "5000000000000000000 B");    
}

#[test]
fn k() {
    assert_eq!(NEGATIVE_5K.to_byteunit(IEC).pow(K), "-4882.81 KiB");  
    assert_eq!(POSITIVE_5K.to_byteunit(IEC).pow(K), "4882.81 KiB");
    assert_eq!(NEGATIVE_5K.to_byteunit(SI).pow(K), "-5000.00 kB"); 
    assert_eq!(POSITIVE_5K.to_byteunit(SI).pow(K), "5000.00 kB"); 
}

#[test]
fn m() {
    assert_eq!(NEGATIVE_5G.to_byteunit(IEC).pow(M), "-4768.37 MiB");
    assert_eq!(POSITIVE_5G.to_byteunit(IEC).pow(M), "4768.37 MiB");
    assert_eq!(NEGATIVE_5G.to_byteunit(SI).pow(M), "-5000.00 MB");
    assert_eq!(POSITIVE_5G.to_byteunit(SI).pow(M), "5000.00 MB");
}

#[test]
fn g() {
    assert_eq!(NEGATIVE_5T.to_byteunit(IEC).pow(G), "-4656.61 GiB");
    assert_eq!(POSITIVE_5T.to_byteunit(IEC).pow(G), "4656.61 GiB");   
    assert_eq!(NEGATIVE_5T.to_byteunit(SI).pow(G), "-5000.00 GB");
    assert_eq!(POSITIVE_5T.to_byteunit(SI).pow(G), "5000.00 GB");   
}

#[test]
fn t() {
    assert_eq!(NEGATIVE_5P.to_byteunit(IEC).pow(T), "-4547.47 TiB");
    assert_eq!(POSITIVE_5P.to_byteunit(IEC).pow(T), "4547.47 TiB");
    assert_eq!(NEGATIVE_5P.to_byteunit(SI).pow(T), "-5000.00 TB");
    assert_eq!(POSITIVE_5P.to_byteunit(SI).pow(T), "5000.00 TB");
}

#[test]
fn p() {
    assert_eq!(NEGATIVE_5E.to_byteunit(IEC).pow(P), "-4440.89 PiB"); 
    assert_eq!(POSITIVE_5E.to_byteunit(IEC).pow(P), "4440.89 PiB");      
    assert_eq!(NEGATIVE_5E.to_byteunit(SI).pow(P), "-5000.00 PB");  
    assert_eq!(POSITIVE_5E.to_byteunit(SI).pow(P), "5000.00 PB");    
}

#[test]
fn e() {
    assert_eq!(NEGATIVE_5E.to_byteunit(IEC).pow(E), "-4.34 EiB");   
    assert_eq!(POSITIVE_5E.to_byteunit(IEC).pow(E), "4.34 EiB");    
    assert_eq!(NEGATIVE_5E.to_byteunit(SI).pow(E), "-5.00 EB");  
    assert_eq!(POSITIVE_5E.to_byteunit(SI).pow(E), "5.00 EB");  
}

#[test]
fn eq() {
   assert_eq!(POSITIVE_5G.to_byteunit(SI) == POSITIVE_5G.to_byteunit(SI), true); 
   assert_eq!(POSITIVE_5B.to_byteunit(SI) == POSITIVE_5G.to_byteunit(SI), false);
}

#[test]
fn mul() {
    let a = ByteUnit::SI(POSITIVE_5B);
    let b = ByteUnit::SI(POSITIVE_5K);
    let multiplication = a * b;

    assert_eq!(multiplication.to_string(), "25.00 GB");
}

#[test]
fn div() {
    let a = ByteUnit::SI(POSITIVE_5G);
    let b = ByteUnit::SI(POSITIVE_5B);
    let division = a / b;

    assert_eq!(division.to_string(), "1.00 MB");
}

#[test]
fn add() {
    let a = ByteUnit::SI(POSITIVE_5B);
    let b = ByteUnit::SI(POSITIVE_5B);
    let division = a + b;

    assert_eq!(division.to_string(), "10.00 kB");
}

#[test]
fn sub() {
    let a = ByteUnit::SI(POSITIVE_5G);
    let b = ByteUnit::SI(POSITIVE_5G);
    let division = a - b;

    assert_eq!(division.to_string(), "0 B");

}

#[test]
fn partial_cmp() {
    let a = ByteUnit::SI(NEGATIVE_5G);
    let b = ByteUnit::SI(POSITIVE_5G);

    assert_eq!(a > b, false);
    assert_eq!(a < b, true);
    assert_eq!(a >= a, true);
    assert_eq!(b <= b, true);
}

#[test]
fn partial_eq() {
    let a = ByteUnit::SI(POSITIVE_5G);
    let b = ByteUnit::SI(POSITIVE_5G);

    assert_eq!(a == b, true);
    assert_eq!(a != b, false);
}

#[test]
fn into() {
    let a: ByteUnit<i64> = "-4.34 EiB".into();
    let b: ByteUnit<i64> = "-2.50 GB".into();

    assert_eq!(a.to_string(), "-4.34 EiB");
    assert_eq!(b.to_string(), "-2.50 GB");
}
