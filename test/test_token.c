#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "conf.h"
#include "token.h"

static void
test_token_next(const char	**curp,
                enum token_res	  exp_res,
	        enum token_type	  exp_type,
	        size_t		  exp_len,
	        const char	 *exp_cur)
{
	const char *cur = *curp;
	struct token_info token;
	enum token_res res = token_next(&cur, &token);

	if (exp_res == TOKEN_RES_NONE) {
		assert_int(res, ==, TOKEN_RES_NONE);
		assert_ptr_equal(cur, exp_cur);

		*curp = cur;
		return;
	}

	assert_int(res, ==, TOKEN_RES_OK);

	assert_string_equal(cur, exp_cur);

	const char *type_s = token_type_str(token.type);
	const char *exp_type_s = token_type_str(exp_type);
	assert_string_equal(type_s, exp_type_s);
	assert_int(token.type, ==, exp_type);

	assert_size(token.len, ==, exp_len);

	assert_true(strncmp(token.src, cur - exp_len, exp_len) == 0);

	*curp = cur;
}

static MunitResult
test_empty(const MunitParameter params[], void *fixture)
{
	const char *src = "";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src);

	return MUNIT_OK;
}

static MunitResult
test_sym_short(const MunitParameter params[], void *fixture)
{
	const char *src = "foo";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_SYM, 3, src + 3);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 3);

	return MUNIT_OK;
}

static MunitResult
test_sym_limit(const MunitParameter params[], void *fixture)
{
	char src[SYM_MAX_LEN + 1 + 1] = {0};
	memset(src, 'a', SYM_MAX_LEN + 1);

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_ERR, SYM_MAX_LEN + 1,
	    src + SYM_MAX_LEN + 1);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + SYM_MAX_LEN + 1);

	return MUNIT_OK;
}

static MunitResult
test_sym_err(const MunitParameter params[], void *fixture)
{
	const char *src = "foo,,";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_ERR, 5, src + 5);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 5);

	return MUNIT_OK;
}


static MunitResult
test_list_start(const MunitParameter params[], void *fixture)
{
	const char *src = "(";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_START, 1, src + 1);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 1);

	return MUNIT_OK;
}

static MunitResult
test_list_end(const MunitParameter params[], void *fixture)
{
	const char *src = ")";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_END, 1, src + 1);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 1);

	return MUNIT_OK;
}

static MunitResult
test_quoted(const MunitParameter params[], void *fixture)
{
	const char *src = "'(foo)";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_QUOTE,      1, src + 1);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_START, 1, src + 2);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_SYM,        3, src + 5);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_END,   1, src + 6);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 6);

	return MUNIT_OK;
}

static MunitResult
test_err(const MunitParameter params[], void *fixture)
{
	const char *src = "$$$";

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_ERR, 3, src + 3);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 3);

	return MUNIT_OK;
}

static MunitResult
test_multi(const MunitParameter params[], void *fixture)
{
	const char *src =
	    "         "		"\n"	// 0  -  9
	    "nil)     "		"\t"	// 10 - 19
	    "foo      "		"\v"	// 20 - 29
	    "   bar   "		"\r"	// 30 - 39
	    "   ,,,   "		"\n"	// 40 - 49
	    "   baz   "		"\n"	// 50 - 59
	    "()       "		"\n";	// 60 - 69

	const char *cur = src;
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_SYM,       3, src + 10 + 3);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_END,   1, src + 10 + 4);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_SYM,        3, src + 20 + 3);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_SYM,        3, src + 30 + 6);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_ERR,        3, src + 40 + 6);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_SYM,        3, src + 50 + 6);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_START, 1, src + 60 + 1);
	test_token_next(&cur, TOKEN_RES_OK, TOKEN_TYPE_LIST_END,   1, src + 60 + 2);
	test_token_next(&cur, TOKEN_RES_NONE, 0, 0, src + 70);

	return MUNIT_OK;
}

MunitTest token_tests[] = {
	{
		.name		= "/empty",
		.test		= test_empty,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/sym-short",
		.test		= test_sym_short,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/sym-limit",
		.test		= test_sym_limit,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/sym-err",
		.test		= test_sym_err,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/list-start",
		.test		= test_list_start,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/list-end",
		.test		= test_list_end,
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
		.name		= "/err",
		.test		= test_err,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/multi",
		.test		= test_multi,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
