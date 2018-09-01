#include <assert.h>
#include <stdlib.h>

#include "env.h"
#include "sval.h"

#include "fixture.h"

void *
generic_setup(const MunitParameter params[], void *user_data)
{
	struct generic_state *st = malloc(sizeof(*st));
	assert(st != NULL);

	env_init(&st->env);

	/* TODO: random values */
	st->sym0 = sym("sym0");
	st->sym1 = sym("sym1");
	st->sym2 = sym("sym2");

	st->v0 = sym("v0");
	st->v1 = sym("v1");
	st->v2 = sym("v2");

	return st;
}

void
generic_tear_down(void *fixture)
{
	struct generic_state *st = fixture;

	sval_free(st->sym0);
	sval_free(st->sym1);
	sval_free(st->sym2);

	sval_free(st->v0);
	sval_free(st->v1);
	sval_free(st->v2);

	env_destroy(&st->env);
	free(st);
}
