// array.rs
// Showcase how we automatically can interface Rust to KLEE
//

#![no_std]
#![no_main]

use klee_sys::klee_make_symbolic;
use panic_klee as _;

fn sum_first_elements(arr: &[u8], index: usize) -> usize {
    let mut acc: usize = 0;
    for i in 0..index {
        if i > arr.len()-1{
            break
        }
        acc += arr[i as usize] as usize;
    }
    acc
}

#[no_mangle]
fn main() {
    let mut arr = [0u8; 8];
    klee_make_symbolic!(&mut arr, "arr");
    let mut i: usize = 0;
    klee_make_symbolic!(&mut i, "i");
    let b = sum_first_elements(&arr, i);
}

// A) Array indexing is tricky to analyse at compile time.
// Thus Rust (rustc) will inject code for run-time verification
// `panic`ing on index out of range.
//
// (Compare to C/C++, where a "buffer overflow" might pass unnoticed
// causing all sorts of problems.)
//
// Compare the test generated in release `--release` (optimized) to
// test generated in debug/dev mode (un-optimized).
//
// Try to explain in your own words the difference and why?
// (Hint, even if we don't use the result `b`, Rust do not optimize out the call, why?)
//
// [your answer here]
/* 
    when producing tests for unoptimized mode, it is relevant to track exactly where in the code a panic might occur
    whereas in the optimized version the code can be reduced to a more minimalistic version e.g 

    In the unoptimized mode 10 tests are generated where the 9 first pass and the 10:th fail with index out of bounds

    In the optimized/released mode 2 tests are generated where one pass and one fail
*/
//
// B) Fix the code so that you don't get an error.
// (It should still compute the sum of the n first elements
// and return the sum of the whole array if index larger than size/length).
// The fix should be in the function (not on the caller side).
//
// [Git commit "Array B"]
//
// C) In the example, the array is holding only zeroes.
// Figure out a way to make the content symbolic.
// Hints: declare as mutable, you can set the whole array symbolic.
//
// [Git commit "Array C"]
//
// D) Analyze the example using KLEE. Now a new (maybe unexpected) error should occur!
// Notice, the error occurs only in `debug/dev` builds.
//
// Explain what caused the error.
//
// [Git commit "Array D"]
//
// [your answer here]
/* 
    Since acc is a u8 int (1 byte) and the integers in the array also are summed up u8 ints
    it may cause overflow
*/
//
// E) Make a sensible fix to the code.
// Motivate your choice.
//
// [your answer here]
/* 
    Changed the type of acc to usize which cannot overflow by eight u8-integers
*/
//
// [Git commit "Array E"]
//
// F) Learning outcome.
// In this small exercise we have used KLEE to find errors in our code.
// For such small examples you may spot the errors by "eye".
// The problem however becomes increasingly difficult with increased code size.
// In industry it is expected 15-50 errors per 1000 delivered lines of code.
//
// KLEE analysis ensures "defined behavior", thus Rust code passing KLEE
// without errors is ensured to be free of run-time panics.
//
// Describe in your own words how that improves robustness and reliability.
//
// [your answer here]
/* 
    It improves robustness and reliability since KLEE can guarantee that the program wont 
    panic during run-time. It is a very powerful tool since it can detect many more bugs by design 
    than could be detected by other software or humans
*/
//
// [Git commit "Array F"]
