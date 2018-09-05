#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "sval.h"
#include "sval_test.h"

static MunitResult
test_eq_sym(const MunitParameter params[], void *fixture)
{
	sval_t v = sym("foobar");
	sval_t w = sym("foobar");

	assert_sval_eq(v, w);

	sval_free(v);
	sval_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_sym(const MunitParameter params[], void *fixture)
{
	sval_t v = sym("foo");
	sval_t w = sym("bar");

	assert_sval_neq(v, w);

	sval_free(v);
	sval_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_immed_immed(const MunitParameter params[], void *fixture)
{
	sval_t v = err_undef();
	sval_t w = list();

	assert_sval_neq(v, w);

	sval_free(v);
	sval_free(w);

	return MUNIT_OK;
}

static MunitResult
test_neq_immed_boxed(const MunitParameter params[], void *fixture)
{
	sval_t v = sym("foo");
	sval_t w = list();

	assert_sval_neq(v, w);

	sval_free(v);
	sval_free(w);

	return MUNIT_OK;
}

static MunitResult
test_quote(const MunitParameter params[], void *fixture)
{
	sval_t v0 = sym("foo");

	sval_t qv0 = quote(v0);
	assert_sval_neq(v0, qv0);

	sval_t qqv = quote(qv0);
	assert_sval_neq(qv0, qqv);

	sval_t qv1 = unquote(qqv);
	assert_sval_eq(qv0, qv1);

	sval_t v1 = unquote(qv1);
	assert_sval_eq(v0, v1);

	sval_free(qqv);

	return MUNIT_OK;
}

MunitTest sval_tests[] = {
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
		.name		= "/neq-immed-immed",
		.test		= test_neq_immed_immed,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/neq-immed-boxed",
		.test		= test_neq_immed_boxed,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/quote",
		.test		= test_quote,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
