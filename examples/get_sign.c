/*
 * First KLEE tutorial: testing a small function
 * https://klee.github.io/tutorials/testing-function/
 */

#include <klee/klee.h>

int get_sign(int x)
{
    if (x == 0)
        return 0;

    if (x < 0)
        return -1;
    else
        return 1;
}

int main()
{
    int a;
    klee_make_symbolic(&a, sizeof(a), "a");
    int b = 1000 / (a + 5);
    return get_sign(a);
}

// A) Compiling into LLVM bitcode
// > clang -emit-llvm -c get_sign.c
//
// Now you can run Klee on your generated bitcode.
//
// > klee get_sign.bc
//
/*
    [ YOUR ANSWER HERE ]
    KLEE: output directory is "/home/thomas/Documents/GitHub/d7020e_klee/examples/klee-out-0"
    KLEE: Using Z3 solver backend
    KLEE: ERROR: (location information missing) divide by zero
    KLEE: NOTE: now ignoring this error at this location

    KLEE: done: total instructions = 46
    KLEE: done: completed paths = 3
    KLEE: done: partially completed paths = 1
    KLEE: done: generated tests = 4

*/
//
// B) Inspecting the output
//
// > ls klee-last/
//
// [your answer here]
// assembly.ll  info  messages.txt  run.istats  run.stats  test000001.div.err  test000001.kquery
// test000001.ktest  test000002.ktest  test000003.ktest  test000004.ktest  warnings.txt
//
// C) Inspecting the generated test cases
//
// > ktest-tool klee-last/test000001.ktest
//
// What path in the code does this test represent?
//
// [your answer here]

/* 
    It represents the path where get_sign returns -1

    ktest file : 'klee-last/test000001.ktest'
    args       : ['get_sign.bc']
    num objects: 1
    object 0: name: 'a'
    object 0: size: 4
    object 0: data: b'\xfb\xff\xff\xff'
    object 0: hex : 0xfbffffff
    object 0: int : -5
    object 0: uint: 4294967291
    object 0: text: ....

 */

//
// > ktest-tool klee-last/test000002.ktest
//
// What path in the code does this test represent?
//
// [your answer here]

/* 
    It represents the path where get_sign returns 1

    ktest file : 'klee-last/test000002.ktest'
    args       : ['get_sign.bc']
    num objects: 1
    object 0: name: 'a'
    object 0: size: 4
    object 0: data: b'\x00\x00\x00\x10'
    object 0: hex : 0x00000010
    object 0: int : 268435456
    object 0: uint: 268435456
    object 0: text: ....

 */

// > ktest-tool klee-last/test000003.ktest
//
// What path in the code does this test represent?
//
// [your answer here]

/* 
    It represents the path where get_sign returns 0

    ktest file : 'klee-last/test000003.ktest'
    args       : ['get_sign.bc']
    num objects: 1
    object 0: name: 'a'
    object 0: size: 4
    object 0: data: b'\x00\x00\x00\x00'
    object 0: hex : 0x00000000
    object 0: int : 0
    object 0: uint: 0
    object 0: text: ....

 */

//
// D) Replaying a test case
//
// Fist check that includes were installed:
// > ls /usr/local/include
// klee
//
// > ls /usr/local/lib
// klee  libkleeRuntest.so  libkleeRuntest.so.1.0
//
// If you installed Klee using the package manager
// the path might be different:
//
// Using `aur` (arch) files are stored in the system default
// folders, `/usr/include` and `/usr/lib`.
//
// If those are ok, then you can compile for replay:
//
// > clang -I /usr/local/include/ -L /usr/local/lib get_sign.c -l kleeRuntest
//
// Or just
// > clang get_sign.c -l kleeRuntest
//
// If the `include` and `lib` paths are the system defaults.
//
// To replay the first test:
//
// We need to add the libary path so it can be dynamically loaded:
// Depending on shell this might look different:
//
// Under `bash` (and `bash` like shells)
// > export LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH
//
// Under `fish`
// > set -x LD_LIBRARY_PATH /usr/local/lib/:$LD_LIBRARY_PATH
//
// Once again, if using the system default system folders
// you don't need to add anything to `LD_LIBRARY_PATH`.
//
// > KTEST_FILE=klee-last/test000001.ktest ./a.out
//
// Now let's inspect the status (return code), in `bash`:
// $? is the return value (error code) as seen by the shell.
//
// > echo $?
//
// In `fish` you would do
//
// > echo $status
//
// Did the result correspond to the expected path for the test?
//
// [your answer here]
// No I got 136
//
// > KTEST_FILE=klee-last/test000002.ktest ./a.out
//
// Inspect the return code:
//
// Did the result correspond to the expected path for the test?
//
// [your answer here]
// yes I got 1
//
// > KTEST_FILE=klee-last/test000003.ktest ./a.out
//
// Inspect the return code:
//
// Did the result correspond to the expected path for the test?
//
// [your answer here]
// yes I got 0
//
// Why not? Confer to shell error codes:
//
// [your answer here]
/* 
    In the first test I got exit code 136 which indicates floating point error

    In the second test I got exit code 1. Why? dont know.......

    In the third test the exit code was 0 and the execution was successful
 */
//
// D) Debugging
//
// In the above example its kind of hard to see exactly
// what happens. Using `gdb` you single step the program.
//
// First build it with debug symbols (`-g`).
// > clang -g -I /usr/local/include/ -L /usr/local/lib get_sign.c -l kleeRuntest
//
// Or if using system defaults:
// > clang -g get_sign.c -l kleeRuntest
//
// Then start `gdb`:
// > KTEST_FILE=klee-last/test000001.ktest gdb ./a.out
// (gdb) break get_sign
//
// (gdb) run
//
// Now we can inspect the `x` argument by:
// (gdb) print x
//
// What value do you get, and why?
//
// [your answer here]
//  No symbol "x" in current context
//
// Step the code
// > (gdb) next
//
// What path did it take, and why?
//
// [your answer here]
// The program terminated with signal SIGFPE, Arithmetic exception.
// It took the path where get_sign returns -1
//
// Now we can try with another test:
//
// (gdb) set environment KTEST_FILE=klee-last/test000002.ktest
//
// And (re-start) the debug session:
// (gdb) run
//
// Step through the code.
//
// Which path did it take, and why?
//
// [your answer here]
//  The program took the path where it returns 1
//
// And finally:
//
// (gdb) set environment KTEST_FILE=klee-last/test000003.ktest
//
// Which path did it take, and why?
//
// [your answer here]
//
//  It took the path where get_sign returns 0 since
//
// E) Under the hood.
//
// Explain in your own words how
// `klee_make_symbolic(&a, sizeof(a), "a");`
// works when you run `klee` to generate test cases:
//
// [your answer here]
// (hint, mark memory region as symbolic)
/* 
    Dereference the variable 'a' and make it symbolic. Run execution on the memory occupied by a
    in order to find possible test values that causes the program to fail/succeed
 */
//
// Explain in your own words how
// `klee_make_symbolic(&a, sizeof(a), "a");`
// works when you replay test cases:
//
// [your answer here]
/* 
    GDB runs the test cases generated by KLEE
 */
// (hint, KTEST_FILE points to a concrete assignment
// of the memory region)
