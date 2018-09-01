#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "sval.h"
#include "sval_test.h"

struct state {
	sval_t v0;
	sval_t v1;
	sval_t v2;
} st;

static void *
setup(const MunitParameter params[], void *user_data)
{
	st.v0 = sym("val0");
	st.v1 = sym("val1");
	st.v2 = sym("val2");

	return &st;
}

static MunitResult
test_is_list(const MunitParameter params[], void *fixture)
{
	sval_t v = list();

	assert_true(is_list(v));

	sval_free(v);

	return MUNIT_OK;
}

static MunitResult
test_is_pair(const MunitParameter params[], void *fixture)
{
	struct state *st = fixture;

	sval_t l = list();
	assert_false(is_pair(l));

	l = cons(st->v0, l);
	assert_false(is_pair(l));

	l = cons(st->v1, l);
	assert_true(is_pair(l));

	l = cons(st->v2, l);
	assert_false(is_pair(l));

	sval_free(l);

	return MUNIT_OK;
}

static MunitResult
test_eq(const MunitParameter params[], void *fixture)
{
	sval_t l0 = list();
	sval_t l1 = list();
	assert_sval_eq(l0, l1);

	l0 = cons(sym("foo"), l0);
	l1 = cons(sym("foo"), l1);
	assert_sval_eq(l0, l1);

	l0 = cons(sym("bar"), l0);
	l1 = cons(sym("bar"), l1);
	assert_sval_eq(l0, l1);

	sval_free(l0);
	sval_free(l1);

	return MUNIT_OK;
}

static MunitResult
test_neq(const MunitParameter params[], void *fixture)
{
	sval_t l0 = list();
	sval_t l1 = list();

	l0 = cons(sym("foo"), l0);
	l1 = cons(sym("bar"), l1);
	assert_sval_neq(l0, l1);

	l0 = cons(sym("foo"), l0);
	assert_sval_neq(l0, l1);

	sval_free(l0);
	sval_free(l1);

	return MUNIT_OK;
}

static MunitResult
test_cons_car_cdr(const MunitParameter params[], void *fixture)
{
	sval_t l0 = list();

	sval_t v1 = sym("foo");
	sval_t v2 = sym("bar");
	sval_t v3 = sym("baz");

	sval_t l1 = cons(v1, l0);
	assert_sval_eq(car(l1), v1);
	assert_sval_eq(cdr(l1), l0);

	sval_t l2 = cons(v2, l1);
	assert_sval_eq(car(l2), v2);
	assert_sval_eq(car(cdr(l2)), v1);
	assert_sval_eq(cdr(l2), l1);
	assert_sval_eq(cdr(cdr(l2)), l0);

	sval_t l3 = cons(v3, l2);
	assert_sval_eq(car(l3), v3);
	assert_sval_eq(car(cdr(l3)), v2);
	assert_sval_eq(car(cdr(cdr(l3))), v1);
	assert_sval_eq(cdr(l3), l2);
	assert_sval_eq(cdr(cdr(l3)), l1);

	sval_free(l3);

	return MUNIT_OK;
}

static MunitResult
test_reverse_inplace(const MunitParameter params[], void *fixture)
{
	sval_t v0 = sym("foo");
	sval_t v1 = sym("bar");
	sval_t v2 = sym("baz");

	sval_t l = list();
	l = list_reverse_inplace(l);
	assert_sval_eq(l, list());

	l = cons(v0, l);
	l = list_reverse_inplace(l);
	assert_sval_eq(v0, car(l));

	l = cons(v1, l);
	l = list_reverse_inplace(l);
	assert_sval_eq(v0, car(l));
	assert_sval_eq(v1, car(cdr(l)));

	l = cons(v2, l);
	l = list_reverse_inplace(l);
	assert_sval_eq(v1, car(l));
	assert_sval_eq(v0, car(cdr(l)));
	assert_sval_eq(v2, car(cdr(cdr(l))));

	sval_free(l);

	return MUNIT_OK;
}

static MunitResult
test_foreach(const MunitParameter params[], void *fixture)
{
	sval_t v0 = sym("foo");
	sval_t v1 = sym("bar");
	sval_t v2 = sym("baz");
	sval_t vs[] = {
		v0,
		v1,
		v2
	};

	sval_t l = cons(v0, cons(v1, cons(v2, list())));

	size_t i = 0;
	sval_t v;
	sval_t j = l;
	LIST_FOREACH(v, j) {
		assert_sval_eq(v, vs[i]);
		i++;
	}

	assert_size(i, ==, 3);

	sval_free(l);

	return MUNIT_OK;
}

MunitTest sval_list_tests[] = {
	{
		.name		= "/is-list",
		.test		= test_is_list,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/is-pair",
		.test		= test_is_pair,
		.setup		= setup,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/eq",
		.test		= test_eq,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/neq",
		.test		= test_neq,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/cons-car-cdr",
		.test		= test_cons_car_cdr,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/reverse-inplace",
		.test		= test_reverse_inplace,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		.name		= "/foreach",
		.test		= test_foreach,
		.setup		= NULL,
		.tear_down	= NULL,
		.options	= MUNIT_TEST_OPTION_NONE,
		.parameters	= NULL
	}, {
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
