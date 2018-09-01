CC=		clang

CFLAGS=		-std=c99 \
		-Wall \
		-Wpedantic \
		-pedantic-errors \
		-Wall \
		-Wextra \
		-O0 \
		-g \
		-Wno-unused-parameter

LDFLAGS=	-lreadline

SRC=		main.c \
		repl.c

CORE_SRC=	builtin.c \
		env.c \
		err.c \
		eval.c \
		lambda.c \
		list.c \
		parse.c \
		sym.c \
		token.c \
		token_debug.c \
		sval.c \
		sval_debug.c \
		sds.c

TEST_SRC=	test/main.c \
		test/fixture.c \
		test/test_env.c \
		test/test_eval.c \
		test/test_lambda.c \
		test/test_list.c \
		test/test_parse.c \
		test/test_token.c \
		test/test_sval.c \
		test/test_sym.c \
		test/munit.c

DEPS_LINKS=	test/munit.c \
		test/munit.h \
		sds.c \
		sds.h \
		sdsalloc.h \
		uthash.h

BIN=		solang

TEST_BIN=	solang_test

.PHONY: all clean clean_deps deps_links

all: $(BIN)

$(BIN): $(SRC) $(CORE_SRC)
	$(CC) -o $(BIN) $(CFLAGS) $(SRC) $(CORE_SRC) $(LDFLAGS)

$(TEST_BIN): $(TEST_SRC) $(CORE_SRC)
	$(CC) -I ./ -o $(TEST_BIN) $(CFLAGS) $(CORE_SRC) $(TEST_SRC) \
		-Wno-missing-field-initializers

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

clean:
	rm -f $(BIN) $(TEST_BIN)

clean_deps: clean_deps_links
	rm -rf deps/

clean_deps_links:
	rm -f $(DEPS_LINKS)
