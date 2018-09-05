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
   * [Static analysis](#static-analysis)
   * [Check memory leaks](#check-memory-leaks)
      * [in main executable](#in-main-executable)
      * [in tests](#in-tests)
   * [Fuzzing](#fuzzing)
      * [1. Build instrumented binary](#1-build-instrumented-binary)
      * [2. Run fuzzer](#2-run-fuzzer)
      * [3. Later on, check crashes one by one by pressing enter](#3-later-on-check-crashes-one-by-one-by-pressing-enter)
   * [Code coverage (for core source files)](#code-coverage-for-core-source-files)
      * [Generate and open report in browser](#generate-and-open-report-in-browser)
      * [Print a summary](#print-a-summary)

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

    $ afl-fuzz -i fuzz/testcases/ -o fuzz/findings ./solang @@

### 3. Later on, check crashes one by one by pressing enter

    $ for f in $(find fuzz/findings/crashes/id* -type f); do ./solang $f; echo $f; read; done

--------------------------------------------------------------------------------

## Code coverage (for core source files)

Clang/LLVM version 4.0 or greater is required for this.

### Generate and open report in browser

    $ make coverage_report

### Print a summary

    $ make coverage_summary
