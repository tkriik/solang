CC=		cc
CFLAGS=		-std=gnu99 -Wall -Wextra -O0 -g -Wno-unused-parameter
LDFLAGS=	-lreadline

SRC=		tal.c \
		tal.h \
		repl.c \
		repl.h

CORE_SRC=	sym.c \
		sym.h \
		token.c \
		token.h \
		val.c \
		val.h

TEST_SRC=	test/tal_test.c \
		test/test_sym.c \
		test/test_token.c

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

test: $(TEST_BIN)

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

clean:
	rm -f $(BIN) $(TEST_BIN)

clean_deps:
	rm -rf deps/
	rm -f $(DEPS_LINKS)
