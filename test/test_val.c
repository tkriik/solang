#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "val.h"
#include "val_test.h"

static MunitResult
test_is_null(const MunitParameter params[], void *fixture)
{
	val_t v = mk_null();

	assert_true(is_null(v));

	return MUNIT_OK;
}

static MunitResult
test_eq_null(const MunitParameter params[], void *fixture)
{
	val_t v = mk_null();
	val_t w = mk_null();

	assert_val_eq(v, w);

	return MUNIT_OK;
}

static MunitResult
test_eq_sym(const MunitParameter params[], void *fixture)
{
	val_t v = mk_sym("foobar", 6);
	val_t w = mk_sym("foobar", 6);

	assert_val_eq(v, w);

	val_free(v);
	val_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_null(const MunitParameter params[], void *fixture)
{
	val_t v = mk_null();
	val_t w = mk_sym("foobar", 6);

	assert_val_neq(v, w);

	val_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_sym(const MunitParameter params[], void *fixture)
{
	val_t v = mk_sym("foo", 3);
	val_t w = mk_sym("bar", 3);

	assert_val_neq(v, w);

	val_free(v);
	val_free(w);

	return MUNIT_OK;
}

MunitTest val_tests[] = {
	{
		.name		= "/is-null",
		.test		= test_is_null,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/eq-null",
		.test		= test_eq_null,
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
		.name		= "/neq-null",
		.test		= test_neq_null,
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
