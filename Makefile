CC=		gcc
CFLAGS=		-std=c99 -Wall -Wextra -O0 -g -Wno-unused-parameter
LDFLAGS=	-lreadline

SRC=		tal.c \
		tal.h \
		repl.c \
		repl.h

CORE_SRC=	list.c \
		parse.c \
		parse.h \
		token.c \
		token.h \
		token_debug.c \
		val.c \
		val.h \
		val_assert.c \
		val_debug.c \
		val_sym.c \
		val_util.c

TEST_SRC=	test/tal_test.c \
		test/test_parse.c \
		test/test_token.c \
		test/test_val.c \
		test/test_val_list.c \
		test/test_val_sym.c \
		test/val_test.h

DEPS_LINKS=	test/munit.c \
		test/munit.h \
		sds.c \
		sds.h \
		sdsalloc.h

BIN=		tal

TEST_BIN=	tal_test

.PHONY: all clean clean_deps deps_links

all: $(BIN)

$(BIN): $(SRC) $(CORE_SRC)
	$(CC) -o $(BIN) $(CFLAGS) $(SRC) $(CORE_SRC) $(DEPS_LINKS) $(LDFLAGS)

$(TEST_BIN): $(TEST_SRC) $(CORE_SRC)
	$(CC) -I ./ -o $(TEST_BIN) $(CFLAGS) $(CORE_SRC) $(TEST_SRC) $(DEPS_LINKS)

deps_links:
	ln -sf ../deps/munit/munit.c test/munit.c
	ln -sf ../deps/munit/munit.h test/munit.h
	ln -sf deps/sds/sds.c sds.c
	ln -sf deps/sds/sds.h sds.h
	ln -sf deps/sds/sdsalloc.h sdsalloc.h

deps/munit:
	git clone https://github.com/nemequ/munit.git deps/munit

deps/sds:
	git clone https://github.com/antirez/sds.git deps/sds

deps: deps/munit deps/sds deps_links

test: $(TEST_BIN)

clean:
	rm -f $(BIN) $(TEST_BIN)

clean_deps:
	rm -rf deps/
	rm -f $(DEPS_LINKS)
