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

pub fn parse(s: &str) -> Result<(f64, f64, i8, bool), Error> {
    let v = match s.to_lowercase() {
        string if string.ends_with("kib") => ("kib", K, true),
        string if string.ends_with("mib") => ("mib", M, true),
        string if string.ends_with("gib") => ("gib", G, true),
        string if string.ends_with("tib") => ("tib", T, true),
        string if string.ends_with("pib") => ("pib", P, true),
        string if string.ends_with("eib") => ("eib", E, true),
        string if string.ends_with("kb") => ("kb", K, false),
        string if string.ends_with("mb") => ("mb", M, false),
        string if string.ends_with("gb") => ("gb", G, false),
        string if string.ends_with("tb") => ("tb", T, false),
        string if string.ends_with("pb") => ("pb", P, false),
        string if string.ends_with("eb") => ("eb", E, false),
        string if string.ends_with("b") => ("b", B, false),
        _ => Err(Error::InvalidUnit(format!("'{s}' contains no supported nor valid byteunits.")))?,
    };
    let s = s.to_lowercase().replace(v.0, "");
    let multiplier = match v.2 {
        true => 1024.0,
        false => 1000.0,
    };

    match s.trim().parse() {
        Ok(val) => Ok((val, multiplier, v.1, v.2)),
        Err(_) => Err(Error::ErroroneousInput(format!("'{s}' contains an invalid float or integer value."))),
    }
}
