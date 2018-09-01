#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "env.h"
#include "sval.h"

#include "fixture.h"
#include "sval_test.h"

static MunitResult
test_define_multi(const MunitParameter params[], void *fixture)
{
	struct generic_state *st = fixture;

	sval_t res = env_define(&st->env, st->sym0, st->v0);
	assert_sval_eq(res, st->sym0);

	res = env_define(&st->env, st->sym0, st->v1);
	assert_sval_eq(res, err_undef());

	return MUNIT_OK;
}

static MunitResult
test_define_lookup(const MunitParameter params[], void *fixture)
{
	struct generic_state *st = fixture;

	struct env *env = &st->env;

	sval_t res0;
	sval_t res1;
	sval_t res2;

	assert_sval_eq(err_undef(), env_lookup(env, st->sym0));
	assert_sval_eq(err_undef(), env_lookup(env, st->sym1));
	assert_sval_eq(err_undef(), env_lookup(env, st->sym2));

	env_define(env, st->sym0, st->v0);
	res0 = env_lookup(env, st->sym0);

	assert_sval_eq(res0, st->v0);
	assert_sval_eq(err_undef(), env_lookup(env, st->sym1));
	assert_sval_eq(err_undef(), env_lookup(env, st->sym2));

	env_define(env, st->sym1, st->v1);
	res1 = env_lookup(env, st->sym1);

	assert_sval_eq(res0, st->v0);
	assert_sval_eq(res1, st->v1);
	assert_sval_eq(err_undef(), env_lookup(env, st->sym2));

	env_define(env, st->sym2, st->v2);
	res2 = env_lookup(env, st->sym2);

	assert_sval_eq(res0, st->v0);
	assert_sval_eq(res1, st->v1);
	assert_sval_eq(res2, st->v2);

	sval_t res_redef = env_define(env, st->sym0, st->v2);
	sval_t v_redef = env_lookup(env, st->sym0);
	assert_sval_eq(res_redef, err_undef());
	assert_sval_eq(v_redef, st->v0);

	return MUNIT_OK;
}

/* TODO: test limit */

MunitTest env_tests[] = {
	{
		.name		= "/define-multi",
		.test		= test_define_multi,
		.setup		= generic_setup,
		.tear_down	= generic_tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/define-lookup",
		.test		= test_define_lookup,
		.setup		= generic_setup,
		.tear_down	= generic_tear_down,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
