CC=		cc
CFLAGS=		-std=c99 -pedantic -Wall -Wextra -O0 -Wno-unused-parameter
LDFLAGS=	-lreadline

SRC=		repl.c \
		tal.c

BIN=		tal

all: $(SRC)
	$(CC) -o $(BIN) $(CFLAGS) $(SRC) $(LDFLAGS)
