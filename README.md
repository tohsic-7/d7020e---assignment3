# KLEE tutorial

## Expected Learning Outcomes

- Tools handling (engineering perspective)

  - Installing and testing klee.

  - Manual use of the KLEE tools for C and Rust programs.

  - Simplified usage adopting `klee-sys` and `cargo klee`.

- A first introduction to formal program analysis

  - Symbolic execution as an automated way to test-case generation with (up to 100%) path coverage.

  - Path constraints built as (First Order Logic) formulas checked by an automatic Satisfiability Modulo Theory (SMT) solver.
  
## Setup

We assume you have a linux based system (arch based recommended), we assume you have `yay` installed, but you can also use `pacman` or other package manager.

## KLEE Dependencies

Dependencies for building klee is discussed in [building klee](https://klee.github.io/build-llvm9/).

Under the hood, klee uses a `sat` solver for First Order Logic (FOL). Klee can interface to [z3](https://en.wikipedia.org/wiki/Z3_Theorem_Prover) which is modern and efficient solver using Satisfiability Modulo Theories (SMT). In essence SMT is FOL + built in theories for reasoning on fixed-size bit-vectors, extensional arrays, datatypes, and uninterpreted functions, making it suitable for program analysis.

So first install `z3` on your system (then klee will use that instead of the default solver).

Later, you also need to have `gdb` installed.

### Arch linux

Install `yay` (an `aur` helper for managing `A`rch `U`ser `R`epositories). If you run Manjaro follow [this](https://citizix.com/how-to-install-yayaur-helper-in-manjaro-arch-linux/) link.

Under arch with `yay` installed simply:

```shell
> yay -S z3
```

```shell
> yay -S gdb
```

### Ubuntu (like) systems

```shell
> sudo apt install z3 libz3-4 libz3-cil libz3-dev libz3-java libz3-jni libz3-ocaml-de

> sudo apt install gdb
```

## Install KLEE from `aur` (arch linux)

NOTICE, as of 2021-12-01, current `aur` is out of date and won't work with the system llvm (version 13), please look at the install from source below.

The `aur` package [klee](https://aur.archlinux.org/packages/klee/), installs KLEE in `/usr/bin` (binaries), `/usr/include` (C-include files), and `usr/lib` (libraries). These are the default system folders respectively, so it makes it easier to compile, link, and run the KLEE tools.

```shell
> yay -S klee
> klee -version
KLEE 2.2 (https://klee.github.io)
  Build mode: Release (Asserts: ON)
  Build revision: 5719d2803e93252e5d4613f43afc7db0d72332f1

LLVM (http://llvm.org/):
  LLVM version 11.0.0
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: skylake
```

Notice, if you have previously installed from source, but want to use the `aur` instead you should remove the source installed files found in `/usr/local/bin`, `/usr/local/include` and `/usr/local/lib` (please make sure that you only remove the KLEE related files).

## Install KLEE from source

KLEE assumes a set of dependencies for building. Under Arch you can install them by.

```shell
> yay -S llvm llvm-libs clang compiler-rt
```

The instructions [building klee](https://klee.github.io/build-llvm9/) is for LLVM 9, but the current master supports LLVM 12, as of 2021-12-01 Arch ships LLVM 13.

As the system LLVM version must match the KLEE version, we have to downgrade the LLVM related tools. You can use the `downgrade` tool to do that.

```shell
> downgrade -S llvm llvm-libs clang compiler-rt --ala-only
```

Choose the latest 12.x.y for each package to downgrade.

Now you can continue installing KLEE from source:

```shell
> git clone https://github.com/klee/klee.git
> cd klee
> mkdir build
> cd build
> cmake ..
> make -j 8 (-j sets number of parallel builds, e.g., on a 8 threaded machine)
> sudo make install
```

Verify that you have the tool installed.

```shell
> klee -version
KLEE 2.3-pre (https://klee.github.io)
  Build mode: RelWithDebInfo (Asserts: ON)
  Build revision: f4c4f164a2d9132fcc53c0ce44ea8c5379d4d93e

LLVM (http://llvm.org/):
  LLVM version 12.0.1
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: znver2

```

If your build fails at some point, consult the docs [building klee](https://klee.github.io/build-llvm9/).

---

## Testing a small C function

See the `examples/get_sign` file.

Here you learn:

- how to generate LLVM-IR
- how to run KLEE
- how to inspect generated test
- how to replay test cases

## Testing a small Rust function

See the `examples/get_sign.rs` file.

Here you learn:

- how to generate LLVM-IR from a Rust program
- how to run KLEE on Rust code
- how to replay test cases for Rust programs

---

## klee-sys and Cargo-klee

See the `cargo-klee-examples` folder.

Here you learn:

- an easy way to compile and run KLEE on Rust code
- an easy way to replay test cases for Rust programs
- an easy way to find "hard to find" errors in embedded code

---

## Related files and their locations and attributes

- `klee-sys`: low-level bindings for KLEE.
  - version: 0.2.0
  - git: `https://gitlab.henriktjader.com/pln/klee-sys`
  - features:
    - `klee-analysis`
      - KLEE API binds to external functions
    - `klee-replay`
      - KLEE API `klee_make_symbolic` binds to inline assembly breakpoint. This allows a debugger to catch the halted CPU and insert test case to location of symbolic object. Other KLEE API binds to Rust `panic!`.

- `cargo-klee`: `cargo` sub-command.
  - version: 0.4.0
  - git: `https://gitlab.henriktjader.com/pln/cargo-klee`

- `panic_klee`: Binds panic handler to external `abort`
  - version: 0.1.0
  - git: `https://gitlab.henriktjader.com/pln/panic-klee`

---

## Why KLEE on Rust

Out the box, Rust provides superior memory safety and guarantees to well defined behavior. However there are cases where the Rust compiler (rustc) cannot statically (at compile time) ensure these guarantees. For such cases (e.g., division by zero, slice/array indexing etc.) Rust injects code for run-time verification that emit a `panic!` (with appropriate error message). While still being safe (the code nevhttps://github.com/perlindgren/vcell.giter runs into memory unsafety or undefined behavior) this limits the reliability (availability) of the system (as its very hard to recover from a `panic!`.) In practice, the developer would typically reboot/reset the system (and store some error code/trace for post-mortem analysis).

With KLEE we can do better! We bind Rust `panic!` to KLEE `abort` (a path termination), and let. For all reachable `panic!`s, KLEE will provide a concrete test, which we can replay and address in our source code. When done, we have a proof of `panic` freedom and thus defined behavior, with huge impact to reliability (and security) of the system at hand.
