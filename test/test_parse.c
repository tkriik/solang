#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "parse.h"
#include "token.h"
#include "val.h"
#include "val_test.h"

static void
test_parse_token(struct token_info *token,
	         enum parse_res exp_res,
	         val_t exp_v)
{
	val_t v;
	enum parse_res res = parse_token(token, &v);

	const char *res_s = parse_res_str(res);
	const char *exp_res_s = parse_res_str(exp_res);
	assert_string_equal(res_s, exp_res_s);
	assert_int(res, ==, exp_res);

	assert_val_eq(v, exp_v);

	val_free(exp_v);
}

static MunitResult
test_null(const MunitParameter params[], void *fixture)
{
	const char *src = "null";
	struct token_info token = {
		.type	= TOKEN_TYPE_NULL,
		.src	= src,
		.len	= 4
	};

	test_parse_token(&token, PARSE_RES_OK, mk_null());

	return MUNIT_OK;
}

static MunitResult
test_sym(const MunitParameter params[], void *fixture)
{
	const char *src = "foo";
	struct token_info token = {
		.type	= TOKEN_TYPE_SYM,
		.src	= src,
		.len	= 3
	};

	test_parse_token(&token, PARSE_RES_OK, mk_sym("foo", 3));

	return MUNIT_OK;
}

static MunitResult
test_err(const MunitParameter params[], void *fixture)
{
	const char *src = ",,,";
	struct token_info token = {
		.type	= TOKEN_TYPE_ERR,
		.src	= src,
		.len	= 3
	};

	test_parse_token(&token, PARSE_RES_ERR, _mk_undef());

	return MUNIT_OK;
}

MunitTest parse_tests[] = {
	{
		.name		= "/null",
		.test		= test_null,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/sym",
		.test		= test_sym,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/err",
		.test		= test_err,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
