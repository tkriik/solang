#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "val.h"
#include "val_test.h"

static MunitResult
test_eq_sym(const MunitParameter params[], void *fixture)
{
	val_t v = sym("foobar");
	val_t w = sym("foobar");

	assert_val_eq(v, w);

	val_free(v);
	val_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_sym(const MunitParameter params[], void *fixture)
{
	val_t v = sym("foo");
	val_t w = sym("bar");

	assert_val_neq(v, w);

	val_free(v);
	val_free(w);

	return MUNIT_OK;
}

MunitTest val_tests[] = {
	{
		.name		= "/eq-sym",
		.test		= test_eq_sym,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/neq-sym",
		.test		= test_neq_sym,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
