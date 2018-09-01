#ifndef SOLANG_TEST_FIXTURE_H
#define SOLANG_TEST_FIXTURE_H

#include "munit.h"

#include "env.h"
#include "sval.h"

struct generic_state {
	struct	env env;

	sval_t	sym0;
	sval_t	sym1;
	sval_t	sym2;

	sval_t	v0;
	sval_t	v1;
	sval_t	v2;
};

void	*generic_setup(const MunitParameter params[], void *user_data);
void	 generic_tear_down(void *);

#endif
