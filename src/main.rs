/*
REPRESENTING DECIMAL NUMBERS
  in this section, we will learn how to pull bits out of a floating-point number and inject those into a single byte
    format of your own creation.
MOTIVATION
Machine learning models often store large arrays of numbers (weights).
Many of these numbers lie in a known, limited range like 0..=1 or -1..=1.
Standard types like f32 (4 bytes) or f64 (8 bytes) can represent much wider ranges than you need.
Idea: If you know the range, you can pack numbers into fewer bytes (sometimes even 1 byte), which saves memory and speeds up storage/transmission.

SO IT IS POSSIBE TO CREATE A DECIMAL NUMBER FORMAT THAT CAN MODEL THAT RANGE CORRECTLY
*/
/*
REPRESENTING FLOATING-POINT NUMBERS
  each floating-point number is laid out in memory as a scientific notation.

  mass of jupiter = 1.898 * 10^27
  mass of ant = 3.801 * 10^-4

  the key insight is that the same number of characters are used to describe vastly different scales

  computer scientists have taken advantage of tht insight to create a fixed-width format that encodes a wide range of numbers.

  each position with in a number in scentific notation is given a role:
    - a sign (negative infinity to 0) - -
    - the mantissa, also known as the significand - 3.801
    - the radix - also know as the base, is the value that is raised to the power of the exponent - 10
    - the exponent describes the scale of the values - -4

    this crosses over to floating point quite neatly. a floating-point value is a container with three fields
      - a sign bit
      - an exponent
      -a mantissa
      where is the radix??? the standard defines it as 2 for all floating-point types. this defination allows the radix to be omitted from the bit pattern itself.

*/
/*
LOOKING INSIDE AN f32
 - the layout is called binary32 (4 bytes of binary numbers)

 Floating-Point Quirks (IEEE-754)

+0 and –0

Two encodings of zero exist:
+0.0 → sign bit = 0
-0.0 → sign bit = 1
They compare as equal (0.0 == -0.0 → true), but can behave differently in some operations:
1.0 / +0.0 = +∞
1.0 / -0.0 = -∞

NaN (Not a Number)

Produced by invalid operations (e.g., 0.0/0.0, √-1).
Many different bit patterns encode NaN.
All NaNs compare as unequal, even to themselves:
NaN == NaN → false
NaN != NaN → true

⚖️ Summary:
+0 and –0 → different bit patterns but compare as equal.
NaN → possibly identical bit patterns but always compare as unequal.
*/

/*
/*
ISOLATING THE SIGN BIT
  to isolate the sign bit, shift the other bits out of the way. for f32, this involves a right shift of 31 places (>>31).

*/
//performa a right shift
fn main() {
    let n:f32  = 42.42;
    let n_bits: u32 = n.to_bits(); //converts the float into its raw 32-bit IEEE - 754 representation

    //the sign bit is the most significant bit (the left corner)
    /*
     all bits are right-shifted 31 places to the right so result -> //00000000 00000000 00000000 00000000   → value 0
     */
    let sign_bit = n_bits >> 31;

    println!("sign-bit : {}", sign_bit);
}
 */

/*
 /*
 ISOLATING THE EXPONENT
to isolate the exponent, two bit manipulations are required. first, perform a right shift to overwrite the mantissa's bits(>>23).
  then use an AND mask (& 0xff) = & 1111_1111 to exclude the right bit.

the exponent's bit also need to go through a decoding step. to decode the exponent, interprate its 8 bits a unsigned integer, the subtract 127 from the result
 unsigned??? - The bias (127) already handles the “negative exponents” range.

  */
  fn main(){
    let n:f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let exponent_ = n_bits >> 23;

    //& to 1 doesnot change any thing. but 0xff is 8 1s so the nighth(the sign bit will be & with 0 so anything & 0 is 0- so & cleared\
    let exponent_ = exponent_ & 0xff;
    let exponent = (exponent_ as i32) - 127;

    println!("exponent : {}", exponent);
}
 */

/*
 /*
  ISOLATING THE MANTISSA
1. Isolating the mantissa

The mantissa (fraction) is stored in the lowest 23 bits of the 32-bit float.
To get just those bits, you mask away the higher ones:
n_bits & 0x7fffff

0x7fffff = binary 0111 1111 1111 1111 1111 1111
This keeps only the lowest 23 bits (mantissa) and clears the sign + exponent.


2. Decoding the mantissa

The mantissa is not stored directly as a normal fraction. It works like this:
Imagine a binary fraction like:
1.m1 m2 m3 m4 ...


where m1, m2, etc. are the mantissa bits.
The implicit leading 1 is always present (except for subnormal numbers).
Then you add fractional values based on the mantissa bits
Each mantissa bit contributes a weight:
The first bit = 2⁻¹ = 0.5
The second bit = 2⁻² = 0.25
The third bit = 2⁻³ = 0.125
… until the 23rd bit = 2⁻²³ ≈ 0.00000011920928955078125
So if mantissa bits were, say, 101..., the contribution would be:
1.0 (implicit) + 0.5 (bit1=1) + 0.0 (bit2=0) + 0.125 (bit3=1) + ...


3. Special cases

The exponent determines whether we treat the mantissa normally:
Exponent all 0s → subnormal number
No implicit 1.0 in front.
Number looks like 0.m1m2m3... × 2^(1−bias)
Lets you represent very small numbers close to zero.
Exponent all 1s → infinity or NaN
If mantissa = 0 → +∞ or −∞ depending on sign bit.
If mantissa ≠ 0 → NaN (Not a Number).


✅ Summary

Mask with 0x7fffff to isolate the 23 mantissa bits.
Decode by summing each bit × its fractional weight, plus an implicit leading 1 (unless exponent=0).
Exponent all 0s → subnormals (no implicit 1).
Exponent all 1s → infinity or NaN.
 */

 /*
  remember - [ sign | 8-bit exponent | 23-bit mantissa ]
  bit#  31      30-23       22-0
*/

fn main() {
  let n:f32 = 42.42;
  let n_bits:u32 = n.to_bits();
  let mut mantissa: f32 = 1.0;
  for i in 0..23{
    let mask = 1 << i;
    let one_at_bit_i = n_bits & mask;

    if one_at_bit_i != 0 {
      let i_ = i as f32;

      // if the bit at ith position is non-zero, calculate its weight
      let weight = 2_f32.powf(i_ - 23.0); // 2_f32 - treats this number as a float (not an integer)
      mantissa += weight;
    }
  }

  println!("mantissa : {}", mantissa);
}
   */

/*
 PARSES RUST'S FLOATING-POINT LITERALS IS HARDER THAN IT LOOKS
 rust's number have methods. to return the nearest integer to 1.2, Rust uses the method 1.2_f32.ceil rather the function call ceil(1.2)
  while often convenient, this can cause some issues when the compiler parses your source code.
    for Eg :- unary minus (-) has lower precedence than method calls, which means unexpected mathematical error can occur. it is often
               helpful to use parentheses to make your intent clear to the compiler. to calculate -1^0, wrap 1.0 in parentheses
               (-1.0_f32).powf(0.0)

               rather than -1.0_f32.powf(0.0) which is interprated as -(1^0). b/c it is mathematically valid, Rust will not complain when parentheses are omitted.


*/

/*
/*
in the following code :
  it extracts the fields from the number 42.42 encoded as an f32 into individual parts, then assembles these again to create another number.
   to convert the bits within a floating-point number to a number, these are three tasks:
    1, extract the bits of those values from the container (to_parts())
    2, decode each value from its raw bit pattern to its actual value(decode())
    3, perform aithmetic operatio to convrt from scientific notation to an ordinary number (from_parts())
*/
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;
fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field   |   as bits   |   as real number");
    println!("sign    |   {:01b}    |   {}", sign, sign_);
    println!("exponent|   {:08b}    |   {}", exp, exp_);
    println!("mantissa|   {:023b}    |   {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1; // stripes 31 unwanted bits away by shifting these nowhere, leaving only the sign bit
    let exponent = (bits >> 23) & 0xff; //0xff = (1111_1111)// stripes 23 unwanted bits away, then filters out the top bit(sign bit) with a logical AND mask
    let fraction = bits & 0x7fffff; //0x7fffff = (0111 1111 1111 1111 1111 1111)  - retains only the 23 least significant bits via and AND mask

    (sign, exponent, fraction) //the mantissa part is called a fraction here as it becomes the mantissa once it's decoded.
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_l = (-1.0_f32).powf(sign as f32); // converts the sign bit to 1.0 or -1.0(-lsign)

    // exponent must become an i32 incase substracting the BIAS results in negative numbers; then it needs to be cast as a f32 so that it can be used for exponentiation
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    // decodes the fraction to mantissa
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_1 = fraction & mask;
        if one_at_bit_1 != 0 {
            let weight = 2_f32.powf(i as f32 - 23.0);
            mantissa += weight;
        }
    }
    (signed_l, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

/*
/*
 FIXED POINT NUMBER FORMATS
  in addition to representing decimal numbers with floating-point formats. fixed point is also available.
   these can be useful for representing fractions and are an option for performing calculations on CPU's without calculating the floating point unit(FPU),
    such as micro-controllers. unlike floating-point numbers, the decimal places doenot move to dynaimcally accomodate different ranges.
  in our case we will be using a fixe-point number format to compactly represent values b/n -1..=1. althought it loses accuracy, it saves significant space.

   The Q format is a fixed-point number format that uses a single byte.3
 It was created by Texas Instruments for embedded computing devices. The specific version of
the Q format that we will implement is called Q7. This indicates that there are 7 bits
available for the represented number plus 1 sign bit. We’ll disguise the decimal
nature of the type by hiding the 7 bits within an i8. That means that the Rust compiler will be able to assist us in keeping track of the value’s sign. We will also be able
to derive traits such as PartialEq and Eq, which provide comparison operators for
our type, for free

*/
 */

/*
Q7 is intended as a compact storage and data transfer type only. its most important role is to convert to and from floating-point types.
  so any number bn/n -128 and 127 (i8) is mapped b/n -1 and 1
*/

use std::convert::From;
/*
 is a trait in which we can define how to convert b/n types
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8); //tuple struct

impl From<f64> for Q7 {
    //convert float64 into Q7
    fn from(n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);

        // coerces any out-of bounds input to fit (there is a loss of information but we choose it instead of panicing)
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        }
        //correct case
        else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        // Q7 = f64 * 128 = f64 * 2^7
        // so f64 = Q7 / 2^7 = Q7 * 2^-7
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

/*
 by design, it is safe to convet from f32 to f64.
  a number that can be represented in 32 bits, can also be represented in 64 bits
*/
impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

/*
 Generally, converting an f64 into a f32 risks a loss of precision. in this application, tht risk doesnot apply
  as we only have numbers between -1 and 1 to convert from
*/
impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q7_from_f64() {
        let a = Q7::from(0.5);
        assert_eq!(a, Q7(64)); // 0.5 * 128 = 64
    }

    #[test]
    fn test_q7_to_f64() {
        let a = Q7(64);
        let b: f64 = a.into();
        assert!((b - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_q7_clamping() {
        assert_eq!(Q7::from(2.0), Q7(127)); // > 1.0
        assert_eq!(Q7::from(-2.0), Q7(-128)); // < -1.0
    }
}
 */
////////////////////////////////////
/*
   GENERATING RANDOM PROBABILITIES FROM RANDOM BYTES

   here is an interesting exercise to test the knowledge that you have developed over the preceding notes and codes.

   imagine that you have a source of random bytes (u8), and you want to convert one of those into a floating-point (f32) value b/n 0 and 1.
    Natively interprating the incoming bytes as f32 / f64 via mem::transmute results in a massive variations in scale.

    the following code demonstrates the division operation that generates a f32 that lies b/n 0 and 1 from an arbitrary input byte.
*/

// fn mock_rand(n: u8) -> f32 {
//     (n as f32) / 255.0 // 255 is the maximum value that u8 can represent
// }

/*
  as division is slow operation, perhaps there is something faster than simply dividing by the largest value that a byte can represent. Perhaps it is possible
   to assume a constant exponent value, then shift the incoming bits into the mantissa, such as these would form a range b/n 0 and 1.

   with an exponent of -1 represented as ob011111110 (126 in base 10). b/c -1 + BIAS = -1 + 127 = 126

   // Efficient u8 → f32 conversion without division:
// Instead of dividing the byte by 255 (slow), we can construct a float directly
// by setting a fixed exponent and placing the byte into the mantissa.
// Using an exponent of -1 (stored as 126 due to IEEE 754 bias),
// the byte produces a float in the range 0.5..0.998.
// This can be normalized to 0.0..~1.0 with a simple subtraction and multiplication.
// Bit manipulation is faster than floating-point division, making it useful for performance-critical code.


*/

// Converts an 8-bit integer (u8) into a floating-point number (f32) in the range 0.0..~1.0
// Avoids slow division by 255 — uses bit manipulation and IEEE 754 representation
fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b_01111110_00000000000000000000000;

    //  u8 (8 bits) → aligned into the **highest 8 bits of the 23-bit mantissa**
    let large_n = (n as u32) << 15;

    // takes a bitwise OR, merging the base with the input byte
    let f32_bits = base | large_n;

    // interprates f32_bits (which is type u32) as an f32
    let m = f32::from_bits(f32_bits);

    //normalizes the output range
    2.0 * (m - 0.5)
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
