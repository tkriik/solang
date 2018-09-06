CC=			clang

CFLAGS=			-std=c99 \
			-Wall \
			-Wpedantic \
			-pedantic-errors \
			-Wall \
			-Wextra \
			-O0 \
			-g \
			-Wno-unused-parameter

LDFLAGS=		-lreadline

ARCH=			linux

SRC=			main.c \
			repl.c

CORE_SRC=		builtin.c \
			env.c \
			err.c \
			eval.c \
			lambda.c \
			list.c \
			parse.c \
			sym.c \
			sys_$(ARCH).c \
			token.c \
			sval.c \

CORE_DEPS=		sds.c

DEBUG_SRC=		token_debug.c \
			sval_debug.c

TEST_SRC=		test/main.c \
			test/fixture.c \
			test/test_env.c \
			test/test_err.c \
			test/test_eval.c \
			test/test_lambda.c \
			test/test_list.c \
			test/test_parse.c \
			test/test_token.c \
			test/test_sval.c \
			test/test_sym.c \
			test/munit.c

DEPS_LINKS=		test/munit.c \
			test/munit.h \
			sds.c \
			sds.h \
			sdsalloc.h \
			uthash.h

BIN=			solang

TEST_BIN=		solang_test

COV_CLANG_VERSION=	4.0
COV_CC=			clang-$(COV_CLANG_VERSION)
COV_BIN=		solang_test_coverage
COV_PROFRAW=		$(COV_BIN).profraw
COV_PROFDATA=		$(COV_BIN).profdata
COV_REPORT=		$(COV_BIN)_report.html

FUZZ_CC=		afl-clang

.PHONY: all clean clean_deps deps_links

all: $(BIN)

$(BIN): $(SRC) $(CORE_SRC) $(CORE_DEPS)
	$(CC) -o $(BIN) $(CFLAGS) $(SRC) $(CORE_SRC) $(DEBUG_SRC) $(CORE_DEPS) $(LDFLAGS)

$(TEST_BIN): $(TEST_SRC) $(CORE_SRC)
	$(CC) -I ./ -o $(TEST_BIN) $(CFLAGS) $(CORE_SRC) $(DEBUG_SRC) $(TEST_SRC) \
		-Wno-missing-field-initializers

$(COV_BIN): $(TEST_SRC) $(CORE_SRC)
	$(COV_CC) -I ./ -o $(COV_BIN) $(CFLAGS) $(CORE_SRC) $(DEBUG_SRC) $(TEST_SRC) \
		-Wno-missing-field-initializers \
		-fprofile-instr-generate \
		-fcoverage-mapping

$(COV_PROFRAW): $(COV_BIN)
	LLVM_PROFILE_FILE="$(COV_PROFRAW)" ./$(COV_BIN)

$(COV_PROFDATA): $(COV_PROFRAW)
	llvm-profdata-$(COV_CLANG_VERSION) merge \
		-sparse $(COV_PROFRAW) \
		-o $(COV_PROFDATA)

deps_links:
	ln -sf ../deps/munit/munit.c test/munit.c
	ln -sf ../deps/munit/munit.h test/munit.h
	ln -sf deps/sds/sds.c sds.c
	ln -sf deps/sds/sds.h sds.h
	ln -sf deps/sds/sdsalloc.h sdsalloc.h
	ln -sf deps/uthash/src/uthash.h uthash.h

deps/munit:
	git clone https://github.com/nemequ/munit.git deps/munit

deps/sds:
	git clone https://github.com/antirez/sds.git deps/sds

deps/uthash:
	git clone https://github.com/troydhanson/uthash.git deps/uthash

deps: deps/munit deps/sds deps/uthash deps_links

test: $(TEST_BIN)

coverage_report: $(COV_BIN) $(COV_PROFDATA)
	llvm-cov-$(COV_CLANG_VERSION) show $(COV_BIN) \
		-instr-profile=$(COV_PROFDATA) \
		-format=html \
		$(CORE_SRC) > $(COV_REPORT)
	sensible-browser $(COV_REPORT)

coverage_summary: $(COV_BIN) $(COV_PROFDATA)
	llvm-cov-$(COV_CLANG_VERSION) report $(COV_BIN) \
		-instr-profile=$(COV_PROFDATA) \
		$(CORE_SRC)

fuzz: CC = afl-clang
fuzz: $(BIN)
	afl-fuzz -i fuzz/testcases -o fuzz/findings ./$(BIN) @@

fuzz_continue: CC = afl-clang
fuzz_continue: $(BIN)
	afl-fuzz -i - -o fuzz/findings ./$(BIN) @@

fuzz_archive:
	mkdir -p fuzz/archive/crashes_$$(git rev-parse HEAD)
	mkdir -p fuzz/archive/hangs_$$(git rev-parse HEAD)
	cp -r fuzz/findings/crashes/id* fuzz/archive/crashes_$$(git rev-parse HEAD)
	cp -r fuzz/findings/hangs/id* fuzz/archive/hangs_$$(git rev-parse HEAD)

clean:
	rm -f $(BIN) $(TEST_BIN) $(COV_BIN)

clean_coverage:
	rm -f $(COV_PROFRAW) $(COV_PROFDATA) $(COV_REPORT)

clean_fuzz:
	rm -rf fuzz/findings/*

clean_deps: clean_deps_links
	rm -rf deps/

clean_deps_links:
	rm -f $(DEPS_LINKS)
