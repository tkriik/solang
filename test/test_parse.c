#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "parse.h"
#include "token.h"
#include "sval.h"
#include "sval_test.h"

static void
test_parse(const char *src, sval_t exp_v)
{
	sval_t v = parse(src);
	assert_sval_eq(v, exp_v);
	sval_free(v);
}

static void
test_parse_err(const char *src)
{
	sval_t v = parse(src);
	assert_sval_eq(v, err_undef());
	sval_free(v);
}

struct parse_fixture {
	const char	*src;
	sval_t		 exp_v;
};

static void
test_parse_fixtures(struct parse_fixture *pfs)
{
	for (struct parse_fixture *pf = pfs; pf->src != NULL; pf++) {
		test_parse(pf->src, pf->exp_v);
		sval_free(pf->exp_v);
	}
}

static MunitResult
test_sym(const MunitParameter params[], void *fixture)
{
	struct parse_fixture pfs[] = {
		{
			.src	= "foo",
			.exp_v	= cons(sym("foo"), list())
		}, {
			.src	= " ->bar",
			.exp_v	= cons(sym("->bar"), list())
		}, {
			.src	= "foo-> ",
			.exp_v	= cons(sym("foo->"), list())
		}, {
			.src	= "\n\r\tbaz9\r\n",
			.exp_v	= cons(sym("baz9"), list())
		}, {
			.src	= NULL
		}
	};

	test_parse_fixtures(pfs);

	return MUNIT_OK;
}

static MunitResult
test_list_0(const MunitParameter params[], void *fixture)
{
	struct parse_fixture pfs[] = {
		{
			.src	= "",
			.exp_v	= list()
		}, {
			.src	= "()",
			.exp_v	= cons(list(), list())
		}, {
			.src	= "(())",
			.exp_v	= cons(cons(list(),
				            list()),
				       list())
		}, {
			.src	= "\n(\t\t(  (\n)\t)\r)\n",
			.exp_v	= cons(cons(cons(list(),
				                 list()),
				            list()),
				       list())
		}, {
			.src	= NULL
		}
	};

	test_parse_fixtures(pfs);

	return MUNIT_OK;
}

static MunitResult
test_list_n(const MunitParameter params[], void *fixture)
{
	struct parse_fixture pfs[] = {
		{
			.src	= "foo bar baz",
			.exp_v	= cons(sym("foo"),
				       cons(sym("bar"),
				            cons(sym("baz"),
				                 list())))
		}, {
			.src	= NULL
		}
	};

	test_parse_fixtures(pfs);

	return MUNIT_OK;
}

static MunitResult
test_list_err(const MunitParameter params[], void *fixture)
{
	const char *srcs[] = {
	    "(",
	    "(foo",
	    "(foo bar",
	    "(foo (bar",
	    "(foo (bar)",
	    "(foo (bar ,))",
	    ")",
	    "foo)",
	    "foo bar)",
	    "foo (bar))",
	    NULL
	};

	for (const char **srcp = srcs; *srcp != NULL; srcp++)
		test_parse_err(*srcp);

	return MUNIT_OK;
}

MunitTest parse_tests[] = {
	{
		.name		= "/sym",
		.test		= test_sym,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/list-0",
		.test		= test_list_0,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/list-n",
		.test		= test_list_n,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/list-err",
		.test		= test_list_err,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
