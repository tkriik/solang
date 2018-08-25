#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "parse.h"
#include "token.h"
#include "val.h"

static MunitResult
test_parse(struct token_info *tokens, size_t ntokens,
    enum parse_result exp_result, val_t exp_v,
    struct token_info *exp_next_tokens)
{
	val_t v;
	struct token_info *next_tokens;
	enum parse_result result = parse(tokens, ntokens, &v, &next_tokens);

	assert_int(result, ==, exp_result);
	assert(is_eq(v, exp_v));
	assert_ptr_equal(next_tokens, exp_next_tokens);

	val_free(v);

	return MUNIT_OK;
}

static MunitResult
test_single_null(const MunitParameter params[], void *fixture)
{
	size_t ntokens = 1;
	struct token_info tokens[1] = {
		{
			.type	= TOKEN_NULL,
			.len	= 4,
			.src	= "null"
		}
	};

	val_t exp_v = mk_null();
	struct token_info *exp_next_tokens = NULL;

	return test_parse(tokens, ntokens, PARSE_OK, exp_v, exp_next_tokens);
}

static MunitResult
test_single_sym(const MunitParameter params[], void *fixture)
{
	size_t ntokens = 1;
	struct token_info tokens[1] = {
		{
			.type	= TOKEN_SYM,
			.len	= 6,
			.src	= "foobar"
		}
	};

	val_t exp_v = mk_sym("foobar", 6);
	struct token_info *exp_next_tokens = NULL;

	return test_parse(tokens, ntokens, PARSE_OK, exp_v, exp_next_tokens);
}

static MunitResult
test_multi(const MunitParameter params[], void *fixture)
{
	const char *src =
	    "foo       "
	    "   null   "
	    "   baz    ";

	val_t exp_v;
	struct token_info *exp_next_tokens;
	MunitResult res;

	size_t ntokens = 3;
	struct token_info tokens[3] = {
		{
			.type	= TOKEN_SYM,
			.len	= 3,
			.src	= src + 0
		}, {
			.type	= TOKEN_NULL,
			.len	= 4,
			.src	= src + 10 + 3
		}, {
			.type	= TOKEN_SYM,
			.len	= 3,
			.src	= src + 10 + 10 + 3
		}
	};

	exp_v = mk_sym("foo", 3);
	exp_next_tokens = &tokens[1];
	res = test_parse(tokens, ntokens, PARSE_CONT, exp_v, exp_next_tokens);
	if (res != MUNIT_OK)
		return res;
	val_free(exp_v);

	exp_v = mk_null();
	exp_next_tokens = &tokens[2];
	res = test_parse(&tokens[1], ntokens - 1, PARSE_CONT, exp_v, exp_next_tokens);
	if (res != MUNIT_OK)
		return res;
	val_free(exp_v);

	exp_v = mk_sym("baz", 3);
	exp_next_tokens = NULL;
	res = test_parse(&tokens[2], ntokens - 2, PARSE_OK, exp_v, exp_next_tokens);
	if (res != MUNIT_OK)
		return res;
	val_free(exp_v);

	return MUNIT_OK;
}

MunitTest parse_tests[] = {
	{
		.name		= "/single-null",
		.test		= test_single_null,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/single-sym",
		.test		= test_single_sym,
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
