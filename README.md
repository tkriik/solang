# Solang (Solid Language)

[![Build Status](https://travis-ci.com/tkriik/solang.svg?branch=master)](https://travis-ci.com/tkriik/solang)

LISP attempt, do not use in production.

## Table of Contents

* [Status](#status)
   * [Features](#features)
   * [Missing features](#missing-features)
* [Dependencies](#dependencies)
   * [System dependencies](#system-dependencies)
   * [Local dependencies](#local-dependencies)
* [Building](#building)
   * [Install system dependencies](#install-system-dependencies)
      * [APT](#apt)
   * [Install local dependencies](#install-local-dependencies)
   * [Compile](#compile)
      * [with Clang](#with-clang)
      * [with GCC](#with-gcc)
   * [Run shell](#run-shell)
   * [Clean project](#clean-project)
   * [Clean local dependencies](#clean-local-dependencies)
* [Development](#development)
   * [General](#general)
      * [Philosophy](#philosophy)
      * [Rules](#rules)
      * [Code style](#code-style)
   * [Testing](#testing)
      * [Build tests](#build-tests)
      * [Run tests](#run-tests)
      * [Print test options](#print-test-options)
   * [Code coverage (for core source files)](#code-coverage-for-core-source-files)
      * [Generate and open report in browser](#generate-and-open-report-in-browser)
      * [Print a summary](#print-a-summary)
   * [Check memory leaks](#check-memory-leaks)
      * [in main executable](#in-main-executable)
      * [in tests](#in-tests)
   * [Static analysis](#static-analysis)
   * [Fuzzing](#fuzzing)
      * [Build and run fuzzer](#build-and-run-fuzzer)
      * [Continue fuzzing](#continue-fuzzing)
      * [Check crashes one by one](#check-crashes-one-by-one)
      * [Archive fuzz results](#archive-fuzz-results)
      * [Clean fuzz results](#clean-fuzz-results)

--------------------------------------------------------------------------------

# Status

Work in progress

## Features

  - Shell
  - Symbols
  - Lists

## Missing features

  - Everything else

--------------------------------------------------------------------------------

# Dependencies

## System dependencies

  - Clang/GCC
  - GNU Readline

## Local dependencies

  - [µnit](https://github.com/nemequ/munit)
  - [SDS](https://github.com/antirez/sds)
  - [uthash](https://troydhanson.github.io/uthash/)

--------------------------------------------------------------------------------

# Building

## Install system dependencies

### APT

    $ apt-get install -y llvm-4.0 libreadline-dev

## Install local dependencies

    $ make deps

## Compile

### with Clang

    $ make

### with GCC

    $ make CC=gcc

## Run shell

    $ ./solang

## Clean project

    $ make clean

## Clean local dependencies

    $ make clean_deps

--------------------------------------------------------------------------------

# Development

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

## Code coverage (for core source files)

Clang/LLVM version 4.0 or greater is required for this.

### Generate and open report in browser

    $ make coverage_report

### Print a summary

    $ make coverage_summary

--------------------------------------------------------------------------------

## Check memory leaks

Install [Valgrind](http://valgrind.org/)

### in main executable

    $ valgrind --leak-check=full ./solang

### in tests

    $ valgrind --leak-check=full ./solang_test --no-fork

**Note**: SDS strings are are detected as *possibly* lost by Valgrind.

--------------------------------------------------------------------------------

## Static analysis

    $ make clean && scan-build make

--------------------------------------------------------------------------------

## Fuzzing

Install [AFL](http://lcamtuf.coredump.cx/afl/).

Initial fuzzing testcases are located under `fuzz/testcases/`,
while the results are stored under `fuzz/findings/`.

### Build and run fuzzer

    $ make fuzz

**Note**: Starting the fuzzer may require system-specific tuning, which
requires superuser rights. If the process doesn't start, follow the
instructions given by the `afl-fuzz` command.

### Continue fuzzing

If you have previous fuzz results that you want to keep, you can continue
fuzzing with this:

    $ make fuzz_continue

### Check crashes one by one

With this you can press enter to proceed through each crash file:

    $ for f in $(find fuzz/findings/crashes/id* -type f); do ./solang $f; echo $f; read; done

### Archive fuzz results

The following command stores current fuzz findings under directories
`fuzz/archive/crashes_$COMMIT_HASH` and `fuzz/archive/hangs_$COMMIT_HASH`:

    $ make fuzz_archive

### Clean fuzz results

    $ make clean_fuzz
