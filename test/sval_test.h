#ifndef SOLANG_VAL_TEST_H
#define SOLANG_VAL_TEST_H

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#define assert_sval_eq(v, w)						\
	do {								\
		assert_true(is_eq(v, w));				\
		assert_true(is_eq(w, v));				\
	} while (0)

#define assert_sval_neq(v, w)						\
	do {								\
		assert_false(is_eq(v, w));				\
		assert_false(is_eq(w, v));				\
	} while (0)

#endif
