#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "token.h"

static MunitResult
test_tokens(const char *src, size_t max_tokens,
    struct token_info *exp_tokens, size_t nexp_tokens,
    enum tokenize_result exp_result)
{
	struct token_info tokens[max_tokens];
	memset(tokens, 0, sizeof(tokens));
	size_t ntokens;

	enum tokenize_result result = tokenize(src, tokens, max_tokens, &ntokens);

	assert_int(result, ==, exp_result);
	assert_size(ntokens, ==, nexp_tokens);

	for (size_t i = 0; i < nexp_tokens; i++) {
		struct token_info *token = &tokens[i];
		struct token_info *exp_token = &exp_tokens[i];

		assert_int(token->type, ==, exp_token->type);
		assert_size(token->len, ==, exp_token->len);
		assert_ptr_equal(token->src, exp_token->src);
		assert(strncmp(token->src, exp_token->src, exp_token->len) == 0);
	}

	return MUNIT_OK;
}

static MunitResult
test_empty(const MunitParameter params[], void *fixture)
{
	const char *src = "";
	size_t max_tokens = 64;

	size_t exp_ntokens = 0;
	struct token_info exp_tokens[0];

	return test_tokens(src, max_tokens, exp_tokens, exp_ntokens, TOKENIZE_OK);
}

static MunitResult
test_limit(const MunitParameter params[], void *fixture)
{
	const char *src = "a b c d e f g h";
	size_t max_tokens = 3;

	size_t exp_ntokens = 3;
	struct token_info exp_tokens[3] = {
		{
			.type	= TOKEN_SYM,
			.len	= 1,
			.src	= src + 0
		}, {
			.type	= TOKEN_SYM,
			.len	= 1,
			.src	= src + 2
		}, {
			.type	= TOKEN_SYM,
			.len	= 1,
			.src	= src + 4
		}
	};

	return test_tokens(src, max_tokens, exp_tokens, exp_ntokens, TOKENIZE_LIMIT);
}

static MunitResult
test_single_standalone(const MunitParameter params[], void *fixture)
{
	const char *src = "my-symbol";
	size_t max_tokens = 64;

	size_t exp_ntokens = 1;
	struct token_info exp_tokens[1] = {
		{
			.type	= TOKEN_SYM,
			.len	= 9,
			.src	= src + 0
		}
	};

	return test_tokens(src, max_tokens, exp_tokens, exp_ntokens, TOKENIZE_OK);
}

static MunitResult
test_single_padded(const MunitParameter params[], void *fixture)
{
	const char *src = "  \n\tmy-symbol  \t\n";
	size_t max_tokens = 64;

	size_t exp_ntokens = 1;
	struct token_info exp_tokens[1] = {
		{
			.type	= TOKEN_SYM,
			.len	= 9,
			.src	= src + 4
		}
	};

	return test_tokens(src, max_tokens, exp_tokens, exp_ntokens, TOKENIZE_OK);
}

static MunitResult
test_multi_standalone(const MunitParameter params[], void *fixture)
{
	const char *src = "a ab) 1bc abcd";
	size_t max_tokens = 64;

	size_t exp_ntokens = 4;
	struct token_info exp_tokens[4] = {
		{
			.type	= TOKEN_SYM,
			.len	= 1,
			.src	= src + 0
		}, {
			.type	= TOKEN_ERR,
			.len	= 3,
			.src	= src + 2
		}, {
			.type	= TOKEN_ERR,
			.len	= 3,
			.src	= src + 6
		}, {
			.type	= TOKEN_SYM,
			.len	= 4,
			.src	= src + 10
		}
	};

	return test_tokens(src, max_tokens, exp_tokens, exp_ntokens, TOKENIZE_OK);
}

static MunitResult
test_multi_padded(const MunitParameter params[], void *fixture)
{
	const char *src = "\n \t a\t\n ab   1bc\n\nabcd\t\t ";
	size_t max_tokens = 64;

	size_t exp_ntokens = 4;
	struct token_info exp_tokens[4] = {
		{
			.type	= TOKEN_SYM,
			.len	= 1,
			.src	= src + 4
		}, {
			.type	= TOKEN_SYM,
			.len	= 2,
			.src	= src + 8
		}, {
			.type	= TOKEN_ERR,
			.len	= 3,
			.src	= src + 13
		}, {
			.type	= TOKEN_SYM,
			.len	= 4,
			.src	= src + 18
		}
	};

	return test_tokens(src, max_tokens, exp_tokens, exp_ntokens, TOKENIZE_OK);
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
		.name		= "/limit",
		.test		= test_limit,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/single-standalone",
		.test		= test_single_standalone,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/single-padded",
		.test		= test_single_padded,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/multi-standalone",
		.test		= test_multi_standalone,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/multi-padded",
		.test		= test_multi_padded,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
