CC=		cc
CFLAGS=		-std=c99 -pedantic -Wall -Wextra -O0 -Wno-unused-parameter
LDFLAGS=	-lreadline

SRC=		repl.c \
		tal.c

DEPS_DIR=	deps

DEPS_LINKS=	mpc.c \
		mpc.h

MPC_TAG=	master

BIN=		tal

all: $(SRC) deps_links
	$(CC) -o $(BIN) $(CFLAGS) $(SRC) $(DEPS_LINKS) $(LDFLAGS)

clean:
	rm -f $(BIN)
	rm -rf $(DEPS_DIR)/
	rm -f $(DEPS_LINKS)

deps/mpc:
	git clone -b $(MPC_TAG) https://github.com/orangeduck/mpc.git $(DEPS_DIR)/mpc

deps_dir:
	mkdir -p $(DEPS_DIR)/

deps_links: deps/mpc
	ln -s $(DEPS_DIR)/mpc/mpc.c mpc.c
	ln -s $(DEPS_DIR)/mpc/mpc.h mpc.h

deps: deps_dir deps_links
