// optional_compare.rs
// Shows how you can use KLEE to compare functional equivalence.

#![no_std]
#![no_main]

use klee_sys::klee_assume;
use klee_sys::klee_make_symbolic;
use panic_klee as _;

// specification:
// if index <= arr.len() return the sum of elements 0..index
// else unspecified
//
// the below implementation meets the spec
fn sum_first_elements(arr: &[u8], index: usize) -> u8 {
    let mut acc = 0;
    if index <= arr.len() {
        for i in 0..index {
            acc += arr[i as usize];
        }
    }
    acc
}

// new specification:
// if index <= arr.len() return the sum of elements 0..index
// else return the sum of all elements 0..arr.len()
fn sum_first_elements2(arr: &[u8], index: usize) -> u8 {
    arr[0..index.min(arr.len())]
        .iter()
        .fold(0, |acc, v| acc + v)
}

#[no_mangle]
fn main() {
    let mut arr = [0u8; 8];
    klee_make_symbolic!(&mut arr, "arr");
    for v in arr {
        klee_assume(v < 10);
    }

    let mut i: usize = 0;
    klee_make_symbolic!(&mut i, "i");
    assert! {sum_first_elements(&arr, i) == sum_first_elements2(&arr, i)};
}

// A) Optional assignment
// During software development and maintenance we might want to replace a software component
// with an equivalent component, with better extra functional properties.
// (E.g., faster and/or less memory consuming or more generic).
//
// Another common case is to replace one component by another with a revised specification.
// (E.g., added functionality).
//
// In this example we have revised the specification to better cover the case where
// index > arr.len(), summing all elements.
//
// We can let KLEE verify that functions have the exact same input/output behavior, e.g.,
// assert! {sum_first_elements(&arr, i) == sum_first_elements2(&arr, i)};
//
// Run KLEE to see where it fails. Inspect the generated error.
//
// What is the value of `i` suggested by KLEE?
//
// [your answer here]
//
// B) Obviously since the specification changed we cannot expect KLEE to prove them equal.
// Hence we need to change the assert!.
// If you want you can make separate assertions for the case where
// index is in range and when out of range.
//
// After revising the assertion now run KLEE on your code.
//
// Did you get any errors?
//
// [your answer here] (if you get an error go back to you assertions, revise and try again)
//
// At this point what did KLEE prove to you.
//
// [your answer here]
//
// C) Learning outcome
// If we want proper proofs over programs, KLEE alone is not sufficient, but we can
// get quite far without applying fully fledged formal methods.
//
// We have seen that KLEE is capable of proving input/output equivalence between
// functions given symbolic values.
//
// We have also seen how partial equivalence can be checked.
//
// One related approach is contract based verification based on Hoare logic.
// Under Hoare logic it has been proven that it is sufficient to
// prove each function to its formal specification (contract) individually,
// in order to obtain a proof for the composed system.
//
// Thus, Hoare logic due to component wise verification is very scalable.
// The problem however is that its not automatic. You have to give specifications
// to each and every function used in order for this approach to work.
//
// Moreover you have to manually prove that each implementation adheres
// to its specification. To this end there now exists proof assistants that
// helps in the process (by adopting SMT solvers similarly to what KLEE does).
// Nevertheless, proofs over programs does not come for free, while KLEE
// based verification requires very little from the user.
//
// What are your own thoughts, explain in your own words how you think KLEE
// could be helpful in code development and maintenance.
//
// [your answer here]
//
// [Git commit "Optional Compare"]
