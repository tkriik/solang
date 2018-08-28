#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "val.h"
#include "val_test.h"

static MunitResult
test_is_list(const MunitParameter params[], void *fixture)
{
	val_t v = list();

	assert_true(is_list(v));

	val_free(v);

	return MUNIT_OK;
}

static MunitResult
test_eq(const MunitParameter params[], void *fixture)
{
	val_t l0 = list();
	val_t l1 = list();
	assert_val_eq(l0, l1);

	l0 = cons(nil(), l0);
	l1 = cons(nil(), l1);
	assert_val_eq(l0, l1);

	l0 = cons(sym("foo", 3), l0);
	l1 = cons(sym("foo", 3), l1);
	assert_val_eq(l0, l1);

	val_free(l0);
	val_free(l1);

	return MUNIT_OK;
}

static MunitResult
test_neq(const MunitParameter params[], void *fixture)
{
	val_t l0 = list();
	val_t l1 = list();

	l0 = cons(sym("foo", 3), l0);
	l1 = cons(sym("bar", 3), l1);
	assert_val_neq(l0, l1);

	l0 = cons(sym("foo", 3), l0);
	assert_val_neq(l0, l1);

	val_free(l0);
	val_free(l1);

	return MUNIT_OK;
}

static MunitResult
test_cons_car_cdr(const MunitParameter params[], void *fixture)
{
	val_t l0 = list();

	val_t v1 = sym("foo", 3);
	val_t v2 = sym("bar", 3);
	val_t v3 = sym("baz", 3);

	val_t l1 = cons(v1, l0);
	assert_val_eq(car(l1), v1);
	assert_val_eq(cdr(l1), l0);

	val_t l2 = cons(v2, l1);
	assert_val_eq(car(l2), v2);
	assert_val_eq(car(cdr(l2)), v1);
	assert_val_eq(cdr(l2), l1);
	assert_val_eq(cdr(cdr(l2)), l0);

	val_t l3 = cons(v3, l2);
	assert_val_eq(car(l3), v3);
	assert_val_eq(car(cdr(l3)), v2);
	assert_val_eq(car(cdr(cdr(l3))), v1);
	assert_val_eq(cdr(l3), l2);
	assert_val_eq(cdr(cdr(l3)), l1);

	val_free(l3);

	return MUNIT_OK;
}

static MunitResult
test_reverse_inplace(const MunitParameter params[], void *fixture)
{
	val_t v0 = sym("foo", 3);
	val_t v1 = nil();
	val_t v2 = sym("baz", 3);

	val_t l = list();
	l = list_reverse_inplace(l);
	assert_val_eq(l, list());

	l = cons(v0, l);
	l = list_reverse_inplace(l);
	assert_val_eq(v0, car(l));

	l = cons(v1, l);
	l = list_reverse_inplace(l);
	assert_val_eq(v0, car(l));
	assert_val_eq(v1, car(cdr(l)));

	l = cons(v2, l);
	l = list_reverse_inplace(l);
	assert_val_eq(v1, car(l));
	assert_val_eq(v0, car(cdr(l)));
	assert_val_eq(v2, car(cdr(cdr(l))));

	val_free(l);

	return MUNIT_OK;
}

static MunitResult
test_foreach(const MunitParameter params[], void *fixture)
{
	val_t v0 = sym("foo", 3);
	val_t v1 = nil();
	val_t v2 = sym("baz", 3);
	val_t vs[] = {
		v0,
		v1,
		v2
	};

	val_t l = cons(v0, cons(v1, cons(v2, list())));

	size_t i = 0;
	val_t v;
	val_t j = l;
	LIST_FOREACH(v, j) {
		assert_val_eq(v, vs[i]);
		i++;
	}

	assert_size(i, ==, 3);

	val_free(l);

	return MUNIT_OK;
}

MunitTest val_list_tests[] = {
	{
		.name		= "/is-list",
		.test		= test_is_list,
		.setup		= NULL,
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
