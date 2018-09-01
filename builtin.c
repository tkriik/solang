#include <assert.h>

#include "builtin.h"
#include "eval.h"
#include "sval.h"

static unsigned long builtin_init_calls = 0;

static sval_t
builtin_quote(struct env *env, sval_t args)
{
	assert(env != NULL);

	return quote(args);
}

void
builtin_init(void)
{
	builtin.sym.def = sym("def");

	builtin.sym.quote = sym("quote");
	builtin.lambda.quote = lambda_builtin(builtin_quote, 1);

	builtin_init_calls++;
}

void
builtin_free(void)
{
	if (0 < builtin_init_calls)
		builtin_init_calls--;

	lambda_free_builtin(builtin.lambda.quote);
}
