# Tanel's Language

[![Build Status](https://travis-ci.com/tkriik/tal.svg?branch=master)](https://travis-ci.com/tkriik/tal)

Work in progress

## Dependencies

### System dependencies

  - GCC/Clang
  - GNU Readline

### Local dependencies

  - [µnit](https://github.com/nemequ/munit)
  - [SDS](https://github.com/antirez/sds)

## Development

### Install system dependencies

#### APT

    $ apt-get install -y clang gcc libreadline-dev

### Install local dependencies

    $ make deps

### Build executable (with GCC)

    $ make

### Build executable (with Clang)

    $ CC=clang make

### Build test executable

    $ make test

### Run

    $ ./tal

### Run tests

    $ ./tal_test

### Print test help

    $ ./tal_test --help

### Clean executable

    $ make clean

### Clean local dependencies

    $ make clean_deps

### Code style

C99-compliant code with [OpenBSD style](https://man.openbsd.org/style)
