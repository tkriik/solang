#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "env.h"
#include "eval.h"
#include "repl.h"
#include "sval.h"
#include "sys.h"

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

static void
eval_file(struct env *env, const char *path)
{
	const char *src = sys_mmap_file_private_readonly(path);
	if (src == NULL) {
		fprintf(stderr, "failed to open file %s\n", path);
		exit(1);
	}

	sval_t res = eval_src(env, src);
	sval_debug_out(path, res);

	sval_free(res);
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

	struct env env;
	env_init(&env);

	const char *path = argv[0];
	eval_file(&env, path);

	env_destroy(&env);

	return 0;
}
