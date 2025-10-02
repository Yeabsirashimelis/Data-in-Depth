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