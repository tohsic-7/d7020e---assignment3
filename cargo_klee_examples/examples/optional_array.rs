// optional_array.rs
// Showcase how we can prove properties of programs.
// This is useful to formulate contracts.

#![no_std]
#![no_main]

use klee_sys::klee_assume;
use klee_sys::klee_make_symbolic;
use panic_klee as _;

fn sum_first_elements(arr: &[u8], index: usize) -> u8 {
    let mut acc = 0;
    if index <= arr.len() {
        for i in 0..index {
            acc += arr[i as usize];
        }
    }
    assert! { acc <= 80 };
    // assert! {
    //     if index <= arr.len() {
    //         acc >= [your expression relating `index` here]
    //     } else {
    //         true
    //     }
    // }
    acc
}

#[no_mangle]
fn main() {
    let mut arr = [0u8; 8];
    klee_make_symbolic!(&mut arr, "arr");
    for v in arr {
        klee_assume(v < 10);
        // klee_assume(v > 3);
    }
    let mut i: usize = 0;
    klee_make_symbolic!(&mut i, "i");
    let b = sum_first_elements(&arr, i);
}

// A) Optional assignment
// We can add additional constraints to symbolic values (variables), e.g.
//
// for v in arr {
//   klee_assume(v < 10);
// }
//
// We can also assert properties of our code, e.g.
//
// assert! { acc <= 80 }
//
// Run KLEE on the problem and see that it passes.
//
// Write in your own words why it passes:
//
// [your answer here]
//
// B)
// For the given assertion what should be the maximum assumed value for each element.
//
// Do this on pen/paper first:
//
// [your answer here: v < [your value here]]
//
// Run KLEE on the max assumed value to see that it passes. Did it pass KLEE without errors?
//
// [your answer here]
//
// If [no] go back to your assumption.
//
// As a sanity check add `1` to the assumed value (i.e., v < [your value here] + 1)
//
// Run KLEE on it.
//
// Did it pass without error?
//
// [your answer here]
//
// If [yes] go back and check your assumption.
//
// Add one to your assumed max value, i.e., v < [your value here] + 1.
//
// C)
// Now extend the assumption with an additional constraint, v > 3.
// (Uncomment the corresponding line of code.)
//
// Compute by hand the corresponding minimum value that the `sum_first_elements`
// will produce.
//
// [your answer here]
//
// Add that clause to the assertion, `assert { acc >= [your value here] && acc <= 80 }`
//
// Run KLEE and check for errors.
//
// The only way to make this pass is assuming acc >= 0, which is sort of meaningless.
// We have more information. There is a relationship in between `index` and `acc`,
//
// We can add a new assertion:
//
// assert! {
//   if index <= arr.len() {
//       acc >= [expression relating `index`]
//   } else {
//       true
//   }
// }
//
// Hint, `index as u8` truncates `usize` into `u8`.
//
// [your expression relating `index`]
//
// Now update the code to have this additional assertion.
//
// Verify that it now passes KLEE without errors (if not go back revise your assertion).
//
// D) Learning outcome.
// In this small exercise we have used KLEE to check properties of code, by means of
// "assumptions" and "assertions".
//
// This can be used to formulate contracts on functions/methods/traits etc.
// Assertions in Rust are just Boolean expressions, and can therefore use the full
// language (like in the above an if statement.)
//
// We can see a conditional:
//  if index <= arr.len() { then condition} else {true}
// as roughly corresponding to First Order Logic implication (A -> B)
//
// And correspondingly:
// assert!{ if index <= arr.len() { then condition} else {true}}
//
// Would yield a proof that (A -> B) holds.
//
// If this is not the case KLEE will produce a concrete test that shows why the
// (A -> B) does NOT hold.
//
// Similarly `klee_assume` also takes a Boolean expressions, and can therefore use
// the full Rust language.
//
// Essentially KLEE translates an assumption into a condition under which the
// code remaining code is executed. Using `&&` between assumptions will create false paths
// and hence KLEE will get produce an error:
//
//  invalid klee_assume call (provably false)
//
// You can typically ignore such errors (there is even a KLEE flag for suppressing them).
// (Run `klee --help` to see if you can find the flag.)
//
// In this example we separated the assumptions so the problem did not occur in the first
// place.
//
// Describe in your own words how you think verifiable contracts can help to create
// more robust and reliable systems.
//
// [your own thoughts here]
//
// [Git commit "Optional Array"]
