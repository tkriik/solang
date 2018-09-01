#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "builtin.h"
#include "repl.h"

static void
usage(void)
{
	extern char *__progname;
	fprintf(stderr,
"\n"
"Open shell: %s [OPTIONS]\n"
"\n"
"Run script: %s [OPTIONS] <FILENAME>\n"
"\n"
"Options:\n"
"  -h   Print this help message\n"
"  -i   Open shell (with or without script)\n"
"\n",
	    __progname,
	    __progname);
	exit(1);
}

int
main(int argc, char **argv)
{
	int interactive = 0;

	int opt;
	while ((opt = getopt(argc, argv, "hi")) != -1) {
		switch (opt) {
		case 'i':
			interactive = 1;
			break;
		case 'h':
		default:
			usage();
			break;
		}
	}

	argc -= optind;
	argv += optind;

	if (argc == 0 || interactive)
		repl_enter();

	printf("eval %s\n", argv[0]);

	return 0;
}
