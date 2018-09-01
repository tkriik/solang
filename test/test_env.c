#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "env.h"
#include "sval.h"
#include "sval_test.h"

static MunitResult
test_define_multi(const MunitParameter params[], void *fixture)
{
	struct env env;
	env_init(&env);

	sval_t s = sym("foo");
	sval_t v0 = sym("val0");
	sval_t v1 = sym("val1");

	sval_t res = env_define(&env, s, v0);
	assert_sval_eq(res, s);

	res = env_define(&env, s, v1);
	assert_sval_eq(res, err_undef());

	env_destroy(&env);

	return MUNIT_OK;
}

static MunitResult
test_define_lookup(const MunitParameter params[], void *fixture)
{
	struct env env;
	env_init(&env);

	sval_t sym0 = sym("foo");
	sval_t sym1 = sym("bar");
	sval_t sym2 = sym("baz");

	sval_t v0 = sym("fooval");
	sval_t v1 = sym("barval");
	sval_t v2 = sym("bazval");

	sval_t res0;
	sval_t res1;
	sval_t res2;

	assert_sval_eq(err_undef(), env_lookup(&env, sym0));
	assert_sval_eq(err_undef(), env_lookup(&env, sym1));
	assert_sval_eq(err_undef(), env_lookup(&env, sym2));

	env_define(&env, sym0, v0);
	res0 = env_lookup(&env, sym0);

	assert_sval_eq(res0, v0);
	assert_sval_eq(err_undef(), env_lookup(&env, sym1));
	assert_sval_eq(err_undef(), env_lookup(&env, sym2));

	env_define(&env, sym1, v1);
	res1 = env_lookup(&env, sym1);

	assert_sval_eq(res0, v0);
	assert_sval_eq(res1, v1);
	assert_sval_eq(err_undef(), env_lookup(&env, sym2));

	env_define(&env, sym2, v2);
	res2 = env_lookup(&env, sym2);

	assert_sval_eq(res0, v0);
	assert_sval_eq(res1, v1);
	assert_sval_eq(res2, v2);

	sval_t res_redef = env_define(&env, sym0, v2);
	sval_t v_redef = env_lookup(&env, sym0);
	assert_sval_eq(res_redef, err_undef());
	assert_sval_eq(v_redef, v0);

	env_destroy(&env);

	return MUNIT_OK;
}

/* TODO: test limit */

MunitTest env_tests[] = {
	{
		.name		= "/define-multi",
		.test		= test_define_multi,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/define-lookup",
		.test		= test_define_lookup,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
