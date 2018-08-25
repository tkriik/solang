#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "sym.h"

static MunitResult
test_str(const char *expected)
{
	size_t len = strlen(expected);
	sym_t sym = sym_alloc(expected, len);
	const char *actual = sym_str(sym);

	assert_string_equal(actual, expected);

	return MUNIT_OK;
}

static MunitResult
test_empty(const MunitParameter params[], void *fixture)
{
	return test_str("");
}

static MunitResult
test_short(const MunitParameter params[], void *fixture)
{
	return test_str("short_sym");
}

static MunitResult
test_long(const MunitParameter params[], void *fixture)
{
	char str[4096 + 1];
	memset(str, 0, sizeof(str) - 1);
	str[4096] = '\0';

	return test_str(str);
}

MunitTest sym_tests[] = {
	{
		.name		= "/empty",
		.test		= test_empty,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/short",
		.test		= test_short,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/long",
		.test		= test_long,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
