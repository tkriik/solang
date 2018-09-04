#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "sval.h"

static MunitResult
test_err_undef(const MunitParameter params[], void *fixture)
{
	sval_t v = err_undef();

	assert_true(is_err(v));
	assert_true(is_err_undef(v));
	assert_string_equal("#error<undefined>", err_str(v));

	return MUNIT_OK;
}

static MunitResult
test_err_nomem(const MunitParameter params[], void *fixture)
{
	sval_t v = err_nomem();

	assert_true(is_err(v));
	assert_true(is_err_nomem(v));
	assert_string_equal("#error<out-of-memory>", err_str(v));

	return MUNIT_OK;
}

MunitTest err_tests[] = {
	{
		.name		= "/err-undef",
		.test		= test_err_undef,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/err-nomem",
		.test		= test_err_nomem,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
