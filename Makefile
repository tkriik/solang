CC=		cc
CFLAGS=		-std=gnu99 -Wall -Wextra -O0 -g
LDFLAGS=	-lreadline

SRC=		repl.c \
		sym.c \
		tal.c \
		val.c

DEPS_LINKS=	sds.c \
		sds.h \
		sdsalloc.h

BIN=		tal

.PHONY: all clean clean_deps deps_links

all: $(BIN)

$(BIN): $(SRC)
	$(CC) -o $(BIN) $(CFLAGS) $(SRC) $(DEPS_LINKS) $(LDFLAGS)

deps_links:
	ln -sf deps/sds/sds.c sds.c
	ln -sf deps/sds/sds.h sds.h
	ln -sf deps/sds/sdsalloc.h sdsalloc.h

deps/sds:
	git clone https://github.com/antirez/sds.git deps/sds

deps: deps/sds deps_links

clean:
	rm -f $(BIN)

clean_deps:
	rm -rf deps/
	rm -f $(DEPS_LINKS)
