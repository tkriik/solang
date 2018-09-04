#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "builtin.h"
#include "env.h"
#include "eval.h"
#include "sval.h"

#include "fixture.h"
#include "sval_test.h"

static MunitResult
test_def(const MunitParameter params[], void *fixture)
{
	struct generic_state *st = fixture;

	sval_t res = eval(&st->env, st->sym0);
	assert_sval_eq(err_undef(), res);

	sval_t exp0 = cons(builtin.sym.def, cons(st->sym0, cons(quote(st->v0), list())));
	res = eval(&st->env, exp0);
	assert_sval_eq(st->sym0, res);

	res = eval(&st->env, st->sym0);
	assert_sval_eq(st->v0, res);

	sval_t exp1 = cons(builtin.sym.def, cons(st->sym0, cons(quote(st->v1), list())));
	res = eval(&st->env, exp1);
	assert_sval_eq(err_undef(), res);

	res = eval(&st->env, st->sym0);
	assert_sval_eq(st->v0, res);

	sval_free(exp0);
	sval_free(exp1);

	return MUNIT_OK;
}

static MunitResult
test_quoted(const MunitParameter params[], void *fixture)
{
	struct generic_state *st = fixture;

	sval_t quoted = quote(st->v0);
	sval_t unquoted = eval(&st->env, quoted);

	assert_sval_eq(unquoted, st->v0);

	sval_free(quoted);

	return MUNIT_OK;
}

static MunitResult
test_multi_quoted(const MunitParameter params[], void *fixture)
{
	struct generic_state *st = fixture;

	sval_t quoted0 = quote(st->v0);
	sval_t quoted1 = quote(quoted0);
	sval_t unquoted = eval(&st->env, quoted1);

	assert_sval_eq(unquoted, quoted0);

	sval_free(quoted1);

	return MUNIT_OK;
}

MunitTest eval_tests[] = {
	{
		.name		= "/def",
		.test		= test_def,
		.setup		= generic_setup,
		.tear_down	= generic_tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/quoted",
		.test		= test_quoted,
		.setup		= generic_setup,
		.tear_down	= generic_tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/multi-quoted",
		.test		= test_multi_quoted,
		.setup		= generic_setup,
		.tear_down	= generic_tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
