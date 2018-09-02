# Solang (Solid Language)

[![Build Status](https://travis-ci.com/tkriik/solang.svg?branch=master)](https://travis-ci.com/tkriik/solang)

LISP attempt, do not use in production.

## Status

Work in progress

### Features

  - Shell

### Missing features

  - Everything else

--------------------------------------------------------------------------------

## Dependencies

### System dependencies

  - GCC/Clang
  - GNU Readline

### Local dependencies

  - [µnit](https://github.com/nemequ/munit)
  - [SDS](https://github.com/antirez/sds)
  - [uthash](https://troydhanson.github.io/uthash/)

--------------------------------------------------------------------------------

## Development

### Install system dependencies

#### APT

    $ apt-get install -y clang gcc libreadline-dev

### Install local dependencies

    $ make deps

### Build (with GCC)

    $ make

### Build (with Clang)

    $ make CC=clang

### Build tests

    $ make test

### Run shell

    $ ./solang

### Run tests

    $ ./solang_test

### Print test options

    $ ./solang_test --help

### Clean project

    $ make clean

### Clean local dependencies

    $ make clean_deps

### Code style

C99-compliant code with [OpenBSD style](https://man.openbsd.org/style)

### Tools

  - Clang
  - [AFL](http://lcamtuf.coredump.cx/afl/)
  - [Valgrind](http://valgrind.org/)

### Static analysis with Clang

    $ make clean && scan-build make

### Memory leak checking with Valgrind

#### In shell

    $ valgrind --leak-check=full ./solang

Note: SDS strings are are detected as *possibly* lost by Valgrind.

#### In tests

    $ valgrind --leak-check=full ./solang_test --no-fork

### Fuzzing with AFL

#### Build instrumented binary

    $ make clean && make CC=afl-clang

#### Run fuzzer

    $ afl-fuzz -i fuzz/testcases/ -o fuzz/findings ./solang

#### Check crashes one by one by pressing enter

    $ for f in $(find fuzz/findings/crashes/id* -type f); do cat $f | ./solang; echo $f; read; done

--------------------------------------------------------------------------------
