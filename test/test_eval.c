#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "builtin.h"
#include "env.h"
#include "eval.h"
#include "val.h"
#include "val_test.h"

struct state {
	struct	env env;
	val_t	sym0;
	val_t	v0;
	val_t	v1;
} st;

static void *
setup(const MunitParameter params[], void *user_data)
{
	env_init(&st.env);
	st.sym0 = sym("sym0");
	st.v0 = sym("val0");
	st.v1 = sym("val1");

	return &st;
}

static void 
tear_down(void *fixture)
{
	struct state *st = fixture;
	env_destroy(&st->env);
	val_free(st->sym0);
	val_free(st->v0);
	val_free(st->v1);
}

static MunitResult
test_quoted(const MunitParameter params[], void *fixture)
{
	struct state *st = fixture;

	val_t quoted = quote(st->v0);
	val_t unquoted = eval(&st->env, quoted);

	assert_val_eq(unquoted, st->v0);

	val_free(quoted);

	return MUNIT_OK;
}

static MunitResult
test_def(const MunitParameter params[], void *fixture)
{
	struct state *st = fixture;

	val_t res = eval(&st->env, st->sym0);
	assert_val_eq(err_undef(), res);

	val_t exp0 = cons(builtin.sym.def, cons(st->sym0, cons(quote(st->v0), list())));
	res = eval(&st->env, exp0);
	assert_val_eq(st->sym0, res);

	res = eval(&st->env, st->sym0);
	assert_val_eq(st->v0, res);

	val_t exp1 = cons(builtin.sym.def, cons(st->sym0, cons(quote(st->v1), list())));
	res = eval(&st->env, exp1);
	assert_val_eq(err_undef(), res);

	res = eval(&st->env, st->sym0);
	assert_val_eq(st->v0, res);

	val_free(exp0);
	val_free(exp1);

	return MUNIT_OK;
}

MunitTest eval_tests[] = {
	{
		.name		= "/quoted",
		.test		= test_quoted,
		.setup		= setup,
		.tear_down	= tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/def",
		.test		= test_def,
		.setup		= setup,
		.tear_down	= tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
