# Data in Depth – Chapter Documentation

This repository contains **notes, explanations, and code examples** from the *Data in Depth* chapter.
It focuses on **data representation, integer and floating-point behavior, fixed-point numbers**, and **efficient bit-level manipulations**.


---

## Table of Contents

* [Introduction](#introduction)
* [Data Representation](#data-representation)
* [Integer Types and Operations](#integer-types-and-operations)
* [Floating-Point Numbers](#floating-point-numbers)
* [Fixed-Point Numbers](#fixed-point-numbers)
* [Random Number Generation Tricks](#random-number-generation-tricks)
* [References](#references)

---

## Introduction

This chapter explains how **data is represented in memory** and how to manipulate it efficiently.
It covers **integers, floats, fixed-point formats**, and **bitwise techniques for performance-sensitive tasks**.

---

## Data Representation

* **Integers:** stored in a fixed number of bits (`u8`, `i32`, etc.).
* **Signed integers:** two's complement representation.
* **Unsigned integers:** only positive numbers.
* **Floating-point numbers:** follow IEEE 754 standard, with a **sign**, **exponent**, and **mantissa**.

---

## Integer Types and Operations

* Rust provides: `i8, i16, i32, i64, i128, u8, u16, u32, u64, u128`.
* **Overflow behavior:**

  * Wrapping, saturating, or panicking depending on method.
* **Bitwise operations:** `&`, `|`, `^`, `<<`, `>>` allow low-level manipulation.

---

## Floating-Point Numbers

* **IEEE 754 f32 (32-bit float):**

```rust
/// Float formula:
/// value = (-1)^sign * 1.mantissa * 2^(exponent - 127)
```

* 1-bit **sign**, 8-bit **exponent**, 23-bit **mantissa**
* **Normalized numbers** always have a hidden leading 1
* **Conversions:**

  * Float ↔ Fixed-point (Q7, etc.)
  * Float ↔ Integer using bit manipulation

**Example – 0.5 in f32 bit pattern:**

```rust
/// Base float representing 0.5
let base: u32 = 0b_01111110_00000000000000000000000;
```

---

## Fixed-Point Numbers

* Fixed-point (Q-format) stores numbers using integers.

```rust
/// Q7 format: represents numbers between -1 and 1
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        // Clamp values to the range [-1, 1]
        if n >= 1.0 { Q7(127) }
        else if n <= -1.0 { Q7(-128) }
        else { Q7((n * 128.0) as i8) }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}
```

* Useful for **embedded systems** or hardware without floating-point support.

---

## Random Number Generation Tricks

* Convert `u8` → `f32` **fast without division**:

```rust
/// Converts an 8-bit byte into a float in [0,1] using bit manipulation
fn mock_rand(n: u8) -> f32 {
    /// Base float = 0.5
    let base: u32 = 0b_01111110_00000000000000000000000;

    /// Insert `n` into the top 8 bits of the 23-bit mantissa
    let large_n = (n as u32) << 15;

    /// Merge base with mantissa
    let f32_bits = base | large_n;

    /// Interpret bits as float
    let m = f32::from_bits(f32_bits);

    /// Normalize to [0,1]
    2.0 * (m - 0.5)
}

fn main() {
    println!("max input 0xff -> {:?}", mock_rand(0xff));
    println!("mid input 0x7f -> {:?}", mock_rand(0x7f));
    println!("min input 0x00 -> {:?}", mock_rand(0x00));
}
```

* How it works:

  1. Fixed exponent = -1 → base = 0.5
  2. Byte inserted into mantissa → float `m ∈ [0.5, ~0.998]`
  3. Normalize → `[0.0, ~1.0]`
* **Advantage:** avoids floating-point division, fully uses bit-level operations.

---

## References

* [IEEE 754 Standard for Floating-Point Arithmetic](https://ieeexplore.ieee.org/document/8766229)
* [Rust Primitive Types](https://doc.rust-lang.org/std/)
* Numerical Computing & Fixed-Point Representation
