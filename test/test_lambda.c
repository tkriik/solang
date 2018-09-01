#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "env.h"
#include "val.h"
#include "val_test.h"

static int identity_called;

static val_t
identity(struct env *env, val_t args)
{
	identity_called = 1;

	return car(args);
}

static struct state {
	struct	env env;
	val_t	identity_sym;
	val_t	identity_lambda;
	val_t	args1;
} st;

static void *
setup(const MunitParameter params[], void *user_data)
{
	env_init(&st.env);
	st.identity_sym = sym("identity");
	st.identity_lambda = lambda_builtin(identity, 1);
	st.args1 = cons(sym("val0"), list());

	identity_called = 0;

	return &st;
}

static void
tear_down(void *fixture)
{
	struct state *st = fixture;

	env_destroy(&st->env);
	lambda_free_builtin(st->identity_lambda);
	val_free(st->args1);
}

static MunitResult
test_builtin(const MunitParameter params[], void *fixture)
{
	struct state *st = fixture;

	assert_true(is_lambda_builtin(st->identity_lambda));

	return MUNIT_OK;
}

static MunitResult
test_apply(const MunitParameter params[], void *fixture)
{
	struct state *st = fixture;

	val_t res = lambda_apply(&st->env, st->identity_lambda, st->args1);
	assert_int(identity_called, ==, 1);
	assert_val_eq(res, sym("val0"));

	return MUNIT_OK;
}

MunitTest lambda_tests[] = {
	{
		.name		= "/builtin",
		.test		= test_builtin,
		.setup		= setup,
		.tear_down	= tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/apply",
		.test		= test_apply,
		.setup		= setup,
		.tear_down	= tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
