# Tanel's Language

[![Build Status](https://travis-ci.com/tkriik/tal.svg?branch=master)](https://travis-ci.com/tkriik/tal)

LISP attempt, do not use in production.

--------------------------------------------------------------------------------

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

    $ CC=clang make

### Build tests

    $ make test

### Run shell

    $ ./tal

### Run tests

    $ ./tal_test

### Print test options

    $ ./tal_test --help

### Clean project

    $ make clean

### Clean local dependencies

    $ make clean_deps

### Code style

C99-compliant code with [OpenBSD style](https://man.openbsd.org/style)

### Tools

  - [Valgrind](http://valgrind.org/)

#### Check memory leaks

##### In shell

    $ valgrind --leak-check=full ./tal

Note: SDS strings are are detected as *possibly* lost by Valgrind.

##### In tests

    $ valgrind --leak-check=full ./tal_test --no-fork

--------------------------------------------------------------------------------
