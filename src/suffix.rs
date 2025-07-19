/*
 * SimpleByteUnit
 *
 * Copyright (C) 2023-2025 Xavier Moffett <sapphirus@azorium.net>
 * SPDX-License-Identifier: Apache-2.0
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::simplebyteunit::*;

pub fn suffix<'a, T: Copy>(unit: &ByteUnit<T>, i: i8) -> &'a str {
    match unit {
        ByteUnit::IEC(_) => match i {
            K => "KiB",
            M => "MiB",
            G => "GiB",
            T => "TiB",
            P => "PiB",
            E => "EiB",
            _ => "B",
        },
        ByteUnit::SI(_) => match i {
            K => "kB",
            M => "MB",
            G => "GB",
            T => "TB",
            P => "PB",
            E => "EB",
            _ => "B",
        },
    }
}

pub fn parse(input: &str) -> Result<(f64, f64, i8, bool), Error> {
    let (value, suffix, power, iec) = match input.to_lowercase() {
        value if value.ends_with("kib") => (value, "kib", K, true),
        value if value.ends_with("mib") => (value, "mib", M, true),
        value if value.ends_with("gib") => (value, "gib", G, true),
        value if value.ends_with("tib") => (value, "tib", T, true),
        value if value.ends_with("pib") => (value, "pib", P, true),
        value if value.ends_with("eib") => (value, "eib", E, true),
        value if value.ends_with("kb") => (value, "kb", K, false),
        value if value.ends_with("mb") => (value, "mb", M, false),
        value if value.ends_with("gb") => (value, "gb", G, false),
        value if value.ends_with("tb") => (value, "tb", T, false),
        value if value.ends_with("pb") => (value, "pb", P, false),
        value if value.ends_with("eb") => (value, "eb", E, false),
        value if value.ends_with("b") => (value, "b", B, false),
        _ => Err(Error::InvalidUnit(format!("'{input}' contains no supported nor valid byteunits.")))?,
    };
    let multiplier = match iec {
        true => 1024.0,
        false => 1000.0,
    };

    match value.replace(suffix, "").trim().parse() {
        Ok(val) => Ok((val, multiplier, power, iec)),
        Err(_) => Err(Error::ErroroneousInput(format!("'{input}' contains an invalid float or integer value."))),
    }
}
