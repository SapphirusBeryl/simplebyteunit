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

use crate::simplebyteunit::*;

pub fn prefix<'a, T: Copy>(unit: &ByteUnit<T>, i: i8) -> &'a str {
    match unit {
        ByteUnit::IEC(_) => match i {
            K => "KiB",
            M => "MiB",
            G => "GiB",
            T => "TiB",
            P => "PiB",
            E => "EiB",
            _ => "B"
        },
        ByteUnit::SI(_) => match i {
            K => "kB",
            M => "MB",
            G => "GB",
            T => "TB",
            P => "PB",
            E => "EB",
            _ => "B"
        }
    }
}

pub fn arithmetic<T: Copy>(value: (T, f64), power_of: i8) -> (i8, f64) where i64: From<T> {
    let bytes: i64 = value.0.into(); 
    let diviser = value.1; 
    let positive = bytes > -1;
    let mut bytes: f64 = if positive { bytes } else { -bytes } as f64;
    let mut power = 0;

    while bytes >= diviser && power < power_of {
        bytes /= diviser; 
        power += 1;
    }
 
    match positive { true => (power, bytes), false => (power, -bytes) }
}
