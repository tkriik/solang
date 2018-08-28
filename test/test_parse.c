#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "parse.h"
#include "token.h"
#include "val.h"
#include "val_test.h"

static void
test_parse(const char *src, val_t exp_v)
{
	val_t v = parse(src);
	assert_val_eq(v, exp_v);
	val_free(v);
}

static void
test_parse_err(const char *src)
{
	val_t v = parse(src);
	assert_val_eq(v, _undef());
	val_free(v);
}

struct parse_fixture {
	const char	*src;
	val_t		 exp_v;
};

static void
test_parse_fixtures(struct parse_fixture *pfs)
{
	for (struct parse_fixture *pf = pfs; pf->src != NULL; pf++) {
		test_parse(pf->src, pf->exp_v);
		val_free(pf->exp_v);
	}
}

static MunitResult
test_null(const MunitParameter params[], void *fixture)
{
	struct parse_fixture pfs[] = {
		{
			.src	= "null",
			.exp_v	= list_cons(null(), list())
		}, {
			.src	= " null",
			.exp_v	= list_cons(null(), list())
		}, {
			.src	= "null ",
			.exp_v	= list_cons(null(), list())
		}, {
			.src	= "\n\v\tnull\r\n",
			.exp_v	= list_cons(null(), list())
		}, {
			.src	= NULL
		}
	};

	test_parse_fixtures(pfs);

	return MUNIT_OK;
}

static MunitResult
test_sym(const MunitParameter params[], void *fixture)
{
	struct parse_fixture pfs[] = {
		{
			.src	= "foo",
			.exp_v	= list_cons(sym("foo", 3), list())
		}, {
			.src	= " ->bar",
			.exp_v	= list_cons(sym("->bar", 5), list())
		}, {
			.src	= "foo-> ",
			.exp_v	= list_cons(sym("foo->", 5), list())
		}, {
			.src	= "\n\r\tbaz9\r\n",
			.exp_v	= list_cons(sym("baz9", 4), list())
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
			.exp_v	= list_cons(list(), list())
		}, {
			.src	= "(())",
			.exp_v	= list_cons(list_cons(list(),
				                      list()),
				            list())
		}, {
			.src	= "\n(\t\t(  (\n)\t)\r)\n",
			.exp_v	= list_cons(list_cons(list_cons(list(),
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
			.src	= "foo null baz",
			.exp_v	= list_cons(sym("foo", 3),
				            list_cons(null(),
				                      list_cons(sym("baz", 3),
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
