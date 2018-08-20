#include <stdio.h>

#include "repl.h"

static void
usage(void)
{
	extern char *__progname;

	fprintf(stderr, "usage: %s\n", __progname);
}

int
main(int argc, char **argv)
{
	if (argc != 1) {
		usage();
	}

	repl_enter();

	return 0;
}
