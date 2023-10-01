/*  
 * SimpleByteUnit
 * 
 * Copyright 2023 Xavier R.M.
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::simplebyteunit::*;

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
        _ => Err(Error::InvalidUnit(format!("'{s}' contains no supported nor valid byteunits.")))? 
    };
    let s = s.to_lowercase()
       .replace(v.0, "")
       .replace(" ", "");
    let multiplier = match v.2 { true => 1024.0, false => 1000.0 };

    match s.parse() {
        Ok(val) => Ok((val, multiplier, v.1, v.2)),
        Err(_) => Err(Error::ErroroneousInput(format!("'{s}' contains an invalid float or integer value.")))
    }
}

pub fn arithmetic<T>(input: (f64, f64, i8, bool)) -> (bool, T) where T: From<i64> {
    let iec = input.3;
    let power_of = input.2;
    let multiplier = input.1;
    let mut value = input.0;
    let mut power: i8 = 0;

    while power < power_of {
        value = value * multiplier;
        power += 1;
    }

    (iec, T::from(value as i64))
}
