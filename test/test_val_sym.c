#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "val.h"

static MunitResult
test_is_sym(const MunitParameter params[], void *fixture)
{
	val_t v = sym("foobar", 6);

	assert_true(is_sym(v));

	val_free(v);

	return MUNIT_OK;
}

static void
test_str(const char *expected)
{
	val_t v = sym(expected, strlen(expected));
	const char *actual = get_sym_str(v);
	
	assert_string_equal(expected, actual);

	val_free(v);
}

static MunitResult
test_short(const MunitParameter params[], void *fixture)
{
	test_str("foobar");

	return MUNIT_OK;
}

static MunitResult
test_long(const MunitParameter params[], void *fixture)
{
	char s[4096 + 1] = {0};
	memset(s, 'A', 4096);

	test_str(s);

	return MUNIT_OK;
}

MunitTest val_sym_tests[] = {
	{
		.name		= "/is-sym",
		.test		= test_is_sym,
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
