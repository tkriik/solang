#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "val.h"
#include "val_test.h"

static val_t
stub_fn(struct env *env, val_t args)
{
	return err_undef();
}

static MunitResult
test_builtin(const MunitParameter params[], void *fixture)
{
	val_t lambda = lambda_builtin(0, stub_fn);
	assert_true(is_lambda_builtin(lambda));

	val_free(lambda);

	return MUNIT_OK;
}

MunitTest lambda_tests[] = {
	{
		.name		= "/builtin",
		.test		= test_builtin,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
