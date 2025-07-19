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

pub fn multiplier<T>(input: (f64, f64, i8, bool)) -> (bool, T)
where
    T: From<i64>, {
    let iec = input.3;
    let power_of = input.2;
    let multiplier = input.1;
    let mut value = input.0;
    let mut power: i8 = 0;

    while power < power_of {
        value *= multiplier;
        power += 1;
    }

    (iec, T::from(value as i64))
}

pub fn divisor<T: Copy>(value: (T, f64), power_of: i8) -> (i8, f64)
where
    i64: From<T>, {
    let bytes: i64 = value.0.into();
    let divisor = value.1;
    let positive = bytes > -1;
    let mut bytes: f64 = if positive { bytes } else { -bytes } as f64;
    let mut power = 0;

    while bytes >= divisor && power < power_of {
        bytes /= divisor;
        power += 1;
    }

    match positive {
        true => (power, bytes),
        false => (power, -bytes),
    }
}
