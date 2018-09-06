#include <stdio.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "builtin.h"
#include "env.h"
#include "eval.h"
#include "parse.h"
#include "sys.h"
#include "sval.h"

#include "sval_test.h"

static MunitResult
test_eval_parsed(const char *filename)
{
	char in[1024] = {0};
	char out[1024] = {0};

	snprintf(in, sizeof(in), "test/resources/eval/%s.in", filename);
	snprintf(out, sizeof(out), "test/resources/eval/%s.out", filename);

	const char *in_src = sys_mmap_file_private_readonly(in);
	const char *out_src = sys_mmap_file_private_readonly(out);

	assert(in_src != NULL);
	assert(out_src != NULL);

	struct env env;
	env_init(&env);

	sval_t actual = eval_src(&env, in_src);
	sval_t expected = parse(out_src);

	if (!is_eq(actual, expected) || !is_eq(expected, actual)) {
		sval_debug_out("expected", expected);
		sval_debug_out("actual", actual);
	}

	assert_sval_eq(actual, expected);

	sval_free(actual);
	sval_free(expected);
	env_destroy(&env);

	return MUNIT_OK;
}

static MunitResult
test_eval_exp(const char *filename, sval_t expected)
{
	char in[1024] = {0};

	snprintf(in, sizeof(in), "test/resources/eval/%s.in", filename);

	const char *in_src = sys_mmap_file_private_readonly(in);

	assert(in_src != NULL);

	struct env env;
	env_init(&env);

	sval_t actual = eval_src(&env, in_src);

	if (!is_eq(actual, expected) || !is_eq(expected, actual)) {
		sval_debug_out("expected", expected);
		sval_debug_out("actual", actual);
	}

	assert_sval_eq(actual, expected);

	sval_free(actual);
	sval_free(expected);
	env_destroy(&env);

	return MUNIT_OK;
}

static MunitResult
test_empty_list(const MunitParameter params[], void *fixture)
{
	test_eval_parsed("empty_list");

	return MUNIT_OK;
}

static MunitResult
test_sym_unknown(const MunitParameter params[], void *fixture)
{
	sval_t exp = cons(err_undef(), cons(err_undef(), cons(err_undef(), list())));
	test_eval_exp("sym_unknown", exp);

	return MUNIT_OK;
}

static MunitResult
test_def(const MunitParameter params[], void *fixture)
{
	test_eval_parsed("def");

	return MUNIT_OK;
}

static MunitResult
test_def_exp(const MunitParameter params[], void *fixture)
{
	test_eval_parsed("def_exp");

	return MUNIT_OK;
}

static MunitResult
test_def_inval(const MunitParameter params[], void *fixture)
{
	test_eval_exp("def_inval", cons(err_undef(), list()));

	return MUNIT_OK;
}

static MunitResult
test_def_multi(const MunitParameter params[], void *fixture)
{
	sval_t exp = cons(sym("foo"),
	             cons(sym("a"),
	             cons(err_undef(),
	             cons(sym("a"), list()))));

	test_eval_exp("def_multi", exp);

	return MUNIT_OK;
}

static MunitResult
test_head(const MunitParameter params[], void *fixture)
{
	test_eval_parsed("head");
	test_eval_exp("head_inval", cons(err_undef(),
	                            cons(err_undef(),
				    cons(err_undef(),
				    list()))));

	return MUNIT_OK;
}

static MunitResult
test_tail(const MunitParameter params[], void *fixture)
{
	test_eval_parsed("tail");
	test_eval_exp("tail_inval", cons(err_undef(),
	                            cons(err_undef(),
				    cons(err_undef(),
				    list()))));

	return MUNIT_OK;
}

static MunitResult
test_quoted(const MunitParameter params[], void *fixture)
{
	test_eval_parsed("quoted_sym");
	test_eval_parsed("quoted_list");

	test_eval_parsed("multi_quoted_sym");
	test_eval_parsed("multi_quoted_list");

	return MUNIT_OK;
}

static MunitResult
test_bad_fn(const MunitParameter params[], void *fixture)
{
	test_eval_exp("bad_fn", cons(sym("foo"), cons(err_undef(), list())));

	return MUNIT_OK;
}

MunitTest eval_tests[] = {
	{
		.name		= "/empty-list",
		.test		= test_empty_list,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/sym-unknown",
		.test		= test_sym_unknown,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/def",
		.test		= test_def,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/def_exp",
		.test		= test_def_exp,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/def-inval",
		.test		= test_def_inval,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/def-multi",
		.test		= test_def_multi,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/head",
		.test		= test_head,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/tail",
		.test		= test_tail,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/quoted",
		.test		= test_quoted,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/bad-fn",
		.test		= test_bad_fn,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
