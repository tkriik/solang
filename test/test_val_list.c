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

	l0 = list_cons(null(), l0);
	l1 = list_cons(null(), l1);
	assert_val_eq(l0, l1);

	l0 = list_cons(sym("foo", 3), l0);
	l1 = list_cons(sym("foo", 3), l1);
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

	l0 = list_cons(sym("foo", 3), l0);
	l1 = list_cons(sym("bar", 3), l1);
	assert_val_neq(l0, l1);

	l0 = list_cons(sym("foo", 3), l0);
	assert_val_neq(l0, l1);

	val_free(l0);
	val_free(l1);

	return MUNIT_OK;
}

static MunitResult
test_cons_head_tail(const MunitParameter params[], void *fixture)
{
	val_t l0 = list();

	val_t v1 = sym("foo", 3);
	val_t v2 = sym("bar", 3);
	val_t v3 = sym("baz", 3);

	val_t l1 = list_cons(v1, l0);
	assert_val_eq(list_head(l1), v1);
	assert_val_eq(list_tail(l1), l0);

	val_t l2 = list_cons(v2, l1);
	assert_val_eq(list_head(l2), v2);
	assert_val_eq(list_head(list_tail(l2)), v1);
	assert_val_eq(list_tail(l2), l1);
	assert_val_eq(list_tail(list_tail(l2)), l0);

	val_t l3 = list_cons(v3, l2);
	assert_val_eq(list_head(l3), v3);
	assert_val_eq(list_head(list_tail(l3)), v2);
	assert_val_eq(list_head(list_tail(list_tail(l3))), v1);
	assert_val_eq(list_tail(l3), l2);
	assert_val_eq(list_tail(list_tail(l3)), l1);

	val_free(l3);

	return MUNIT_OK;
}

static MunitResult
test_reverse_inplace(const MunitParameter params[], void *fixture)
{
	val_t v0 = sym("foo", 3);
	val_t v1 = null();
	val_t v2 = sym("baz", 3);

	val_t l = list();
	l = list_reverse_inplace(l);
	assert_val_eq(l, list());

	l = list_cons(v0, l);
	l = list_reverse_inplace(l);
	assert_val_eq(v0, list_head(l));

	l = list_cons(v1, l);
	l = list_reverse_inplace(l);
	assert_val_eq(v0, list_head(l));
	assert_val_eq(v1, list_head(list_tail(l)));

	l = list_cons(v2, l);
	l = list_reverse_inplace(l);
	assert_val_eq(v1, list_head(l));
	assert_val_eq(v0, list_head(list_tail(l)));
	assert_val_eq(v2, list_head(list_tail(list_tail(l))));

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
		.name		= "/cons-head-tail",
		.test		= test_cons_head_tail,
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
		NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL
	}
};
