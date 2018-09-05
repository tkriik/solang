--------------------------------------------------------------------------------
# Solang (Solid Language)
--------------------------------------------------------------------------------

[![Build Status](https://travis-ci.com/tkriik/solang.svg?branch=master)](https://travis-ci.com/tkriik/solang)

LISP attempt, do not use in production.

## Status

Work in progress

### Features

  - Shell
  - Symbols
  - Lists

### Missing features

  - Everything else

--------------------------------------------------------------------------------

## Dependencies

### System dependencies

  - Clang/GCC
  - GNU Readline

### Local dependencies

  - [µnit](https://github.com/nemequ/munit)
  - [SDS](https://github.com/antirez/sds)
  - [uthash](https://troydhanson.github.io/uthash/)

--------------------------------------------------------------------------------

## Building

### Install system dependencies

#### APT

    $ apt-get install -y llvm-4.0 libreadline-dev

### Install local dependencies

    $ make deps

### Compile

#### with Clang

    $ make

#### with GCC

    $ make CC=gcc

### Run shell

    $ ./solang

### Clean project

    $ make clean

### Clean local dependencies

    $ make clean_deps

--------------------------------------------------------------------------------

# Development

--------------------------------------------------------------------------------

## General

### Philosophy

  1. Make it simple
  1. Make it correct
  1. Make it fast

### Rules

  1. Write dumb code
  1. Use asserts for assumptions at all stages
  1. Use unit tests and coverage for known paths
  1. Check those memory leaks
  1. Use static analysis and fuzzing to catch unknown unknowns

### Code style

C99-compliant code with [OpenBSD style](https://man.openbsd.org/style).

--------------------------------------------------------------------------------

## Testing

### Build tests

    $ make test

### Run tests

    $ ./solang_test

### Print test options

    $ ./solang_test --help

--------------------------------------------------------------------------------

## Static analysis

    $ make clean && scan-build make

--------------------------------------------------------------------------------

## Check memory leaks

Install [Valgrind](http://valgrind.org/)

### in main executable

    $ valgrind --leak-check=full ./solang

### in tests

    $ valgrind --leak-check=full ./solang_test --no-fork

**Note**: SDS strings are are detected as *possibly* lost by Valgrind.

--------------------------------------------------------------------------------

## Fuzzing

Install [AFL](http://lcamtuf.coredump.cx/afl/)

### 1. Build instrumented binary

    $ make clean && make CC=afl-clang

### 2. Run fuzzer

    $ afl-fuzz -i fuzz/testcases/ -o fuzz/findings ./solang

### 3. Later on, check crashes one by one by pressing enter

    $ for f in $(find fuzz/findings/crashes/id* -type f); do cat $f | ./solang; echo $f; read; done

--------------------------------------------------------------------------------

## Code coverage (for core source files)

Clang/LLVM version 4.0 or greater is required for this.

### Generate and open report in browser

    $ make coverage_report

### Print a summary

    $ make coverage_summary
