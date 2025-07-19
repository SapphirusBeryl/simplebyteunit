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

/*!
Fast, stupid simple ByteUnit implementation

Provides a simple way to encapsulate primitives as byteunits.
*/
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use crate::{
    arithmetic::{divisor, multiplier},
    suffix::{parse, suffix},
};

/// IEC ByteUnit (x*1024)
pub const IEC: ByteUnit<()> = ByteUnit::IEC(());
/// SI ByteUnit (x*1000)
pub const SI: ByteUnit<()> = ByteUnit::SI(());
/// Power of 6 (Exa/Exbi)
pub const E: i8 = 6;
/// Power of 5 (Peta/Pebi)
pub const P: i8 = 5;
/// Power of 4 (Tera/Tebi)
pub const T: i8 = 4;
/// Power of 3 (Giga/Gibi)
pub const G: i8 = 3;
/// Power of 2 (Mega/Mebi)
pub const M: i8 = 2;
/// Power of 1 (Kilo/Kibi)
pub const K: i8 = 1;
/// Base unit
pub const B: i8 = 0;
/// Maximum supported power
pub const MAX: i8 = E;

/// Thin encapsulate of a supported, primitive integer to provide simple byteunit facilities
pub enum ByteUnit<T: Copy> {
    IEC(T),
    SI(T),
}

#[derive(Debug)]
pub enum Error {
    InvalidUnit(String),
    ErroroneousInput(String),
}

/// Trait providing generic abstraction to encapsulate primitive in a ByteUnit
pub trait ToByteUnit<T: Copy> {
    fn to_byteunit(self, byte: ByteUnit<()>) -> ByteUnit<T>;
}

impl ToByteUnit<u32> for u32
where
    i64: From<u32>,
{
    fn to_byteunit(self, unit: ByteUnit<()>) -> ByteUnit<u32> {
        match unit {
            ByteUnit::IEC(()) => ByteUnit::IEC(self),
            ByteUnit::SI(()) => ByteUnit::SI(self),
        }
    }
}

impl ToByteUnit<i32> for i32
where
    i64: From<i32>,
{
    fn to_byteunit(self, unit: ByteUnit<()>) -> ByteUnit<i32> {
        match unit {
            ByteUnit::IEC(()) => ByteUnit::IEC(self),
            ByteUnit::SI(()) => ByteUnit::SI(self),
        }
    }
}

impl ToByteUnit<i64> for i64
where
    i64: From<i64>,
{
    fn to_byteunit(self, unit: ByteUnit<()>) -> ByteUnit<i64> {
        match unit {
            ByteUnit::IEC(()) => ByteUnit::IEC(self),
            ByteUnit::SI(()) => ByteUnit::SI(self),
        }
    }
}

impl<T> ByteUnit<T>
where
    i64: From<T>,
    T: Copy,
{
    fn value(&self) -> (T, f64) {
        match self {
            Self::IEC(val) => (*val, 1024.0),
            Self::SI(val) => (*val, 1000.0),
        }
    }

    fn format(&self, arithmetic: (i8, f64)) -> String {
        let power = arithmetic.0;
        let value = arithmetic.1;

        match power {
            B => format!("{:.0} {}", value, suffix(self, power)),
            _ => format!("{:.2} {}", value, suffix(self, power)),
        }
    }

    /// Acquire and return base value of encapsulated primitive
    pub fn val(&self) -> T {
        match self {
            Self::IEC(val) => *val,
            Self::SI(val) => *val,
        }
    }

    /// Returns a formatted string with up-to a maximum supported power.
    pub fn max(&self) -> String {
        self.format(divisor(self.value(), MAX))
    }

    /// Returns a formatted string with a maximum of the specified power.
    pub fn pow(&self, power_of: i8) -> String {
        self.format(divisor(self.value(), power_of))
    }

    /// Returns a formatted string with a maximum power of 1 (Kilo/Kibi)
    pub fn k(&self) -> String {
        self.format(divisor(self.value(), K))
    }

    /// Returns a formatted string with a maximum power of 2 (Mega/Mebi)
    pub fn m(&self) -> String {
        self.format(divisor(self.value(), M))
    }

    /// Returns a formatted string with a maximum power of 3 (Giga/Gibi)
    pub fn g(&self) -> String {
        self.format(divisor(self.value(), G))
    }

    /// Returns a formatted string with a maximum power of 4 (Tera/Tebi)
    pub fn p(&self) -> String {
        self.format(divisor(self.value(), P))
    }

    /// Returns a formatted string with a maximum power of 5 (Peta/Pebi)
    pub fn t(&self) -> String {
        self.format(divisor(self.value(), T))
    }

    /// Returns a formatted string with a maximum power of 6 (Exa/Exbi)
    pub fn e(&self) -> String {
        self.format(divisor(self.value(), E))
    }
}

/// Display implementation with a maximum power of 6 (Exa/Exbi)
impl<T> Display for ByteUnit<T>
where
    i64: From<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arithmetic = divisor(self.value(), MAX);
        let bytes = arithmetic.1;
        let index = arithmetic.0;

        match index {
            B => write!(f, "{:.0} {}", bytes, suffix(self, index)),
            _ => write!(f, "{:.2} {}", bytes, suffix(self, index)),
        }
    }
}

/// Debug implementation with a maximum power of 6 (Exa/Exbi)
impl<T> Debug for ByteUnit<T>
where
    i64: From<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arithmetic = divisor(self.value(), MAX);
        let bytes = arithmetic.1;
        let index = arithmetic.0;

        match index {
            B => write!(f, "'{:.0} {}'", bytes, suffix(self, index)),
            _ => write!(f, "'{:.2} {}'", bytes, suffix(self, index)),
        }
    }
}

impl<T> From<&str> for ByteUnit<T>
where
    i64: From<T>,
    T: Copy,
    T: From<i64>,
{
    fn from(value: &str) -> Self {
        ByteUnit::from_str(value).unwrap()
    }
}

impl<T> FromStr for ByteUnit<T>
where
    i64: From<T>,
    T: Copy,
    i64: From<T>,
    T: From<i64>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = multiplier::<T>(parse(s)?);

        match input.0 {
            true => Ok(ByteUnit::IEC(input.1)),
            false => Ok(ByteUnit::SI(input.1)),
        }
    }
}

impl<T> PartialOrd for ByteUnit<T>
where
    i64: From<T>,
    T: Copy + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let value = i64::from(self.val());
        let other = i64::from(other.val());

        value.partial_cmp(&other)
    }
}

impl<T> PartialEq for ByteUnit<T>
where
    i64: From<T>,
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        other.val().eq(&self.val())
    }
}

impl<T> Add for ByteUnit<T>
where
    i64: From<T>,
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, input: Self) -> Self::Output {
        match self {
            Self::IEC(value) => Self::IEC(value + input.val()),
            Self::SI(value) => Self::SI(value + input.val()),
        }
    }
}

impl<T> Sub for ByteUnit<T>
where
    i64: From<T>,
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, input: Self) -> Self::Output {
        match self {
            Self::IEC(value) => Self::IEC(value - input.val()),
            Self::SI(value) => Self::SI(value - input.val()),
        }
    }
}

impl<T> Mul for ByteUnit<T>
where
    i64: From<T>,
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, input: Self) -> Self::Output {
        match self {
            Self::IEC(value) => Self::IEC(value * input.val()),
            Self::SI(value) => Self::SI(value * input.val()),
        }
    }
}

impl<T> Div for ByteUnit<T>
where
    i64: From<T>,
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, input: Self) -> Self::Output {
        match self {
            Self::IEC(value) => Self::IEC(value / input.val()),
            Self::SI(value) => Self::SI(value / input.val()),
        }
    }
}
