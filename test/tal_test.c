#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "tal_test.h"

static const char *DEFAULT_TEST_MODULES[] = {
	"sym",
	NULL
};

static test_module_fn
get_test_module_fn(const char *module)
{
	if (strcmp(module, "sym") == 0)
		return test_sym;
	
	return NULL;
}

static int
validate_modules(const char **modules)
{
	int rc = 0;

	for (const char **module = modules; *module != NULL; module++) {
		test_module_fn fn = get_test_module_fn(*module);
		if (fn == NULL) {
			fprintf(stderr, "no such test module: %s\n", *module);
			rc = 1;
		}
	}

	return rc;
}

static void
run_modules(const char **modules)
{
	for (const char **module = modules; *module != NULL; module++) {
		test_module_fn fn = get_test_module_fn(*module);
		fn();
	}
}

int
main(int argc, const char **argv)
{
	const char **test_modules = DEFAULT_TEST_MODULES;

	if (1 < argc)
		test_modules = &argv[1];

	int rc = validate_modules(test_modules);
	if (rc != 0) {
		fprintf(stderr, "available test modules:");
		for (const char **module = DEFAULT_TEST_MODULES; *module != NULL; module++) {
			fprintf(stderr, " %s", *module);
			fprintf(stderr, "\n");
		}
		return rc;
	}

	run_modules(test_modules);

	return 0;
}
