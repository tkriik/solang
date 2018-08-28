#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "val.h"
#include "val_test.h"

static MunitResult
test_is_nil(const MunitParameter params[], void *fixture)
{
	val_t v = nil();

	assert_true(is_nil(v));

	return MUNIT_OK;
}

static MunitResult
test_eq_nil(const MunitParameter params[], void *fixture)
{
	val_t v = nil();
	val_t w = nil();

	assert_val_eq(v, w);

	return MUNIT_OK;
}

static MunitResult
test_eq_sym(const MunitParameter params[], void *fixture)
{
	val_t v = sym("foobar", 6);
	val_t w = sym("foobar", 6);

	assert_val_eq(v, w);

	val_free(v);
	val_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_nil(const MunitParameter params[], void *fixture)
{
	val_t v = nil();
	val_t w = sym("foobar", 6);

	assert_val_neq(v, w);

	val_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_sym(const MunitParameter params[], void *fixture)
{
	val_t v = sym("foo", 3);
	val_t w = sym("bar", 3);

	assert_val_neq(v, w);

	val_free(v);
	val_free(w);

	return MUNIT_OK;
}

MunitTest val_tests[] = {
	{
		.name		= "/is-nil",
		.test		= test_is_nil,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/eq-nil",
		.test		= test_eq_nil,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/eq-sym",
		.test		= test_eq_sym,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/neq-nil",
		.test		= test_neq_nil,
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
