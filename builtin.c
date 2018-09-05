#include <assert.h>

#include "builtin.h"
#include "eval.h"
#include "sval.h"

struct builtin_info builtin;

static unsigned long builtin_init_calls = 0;

static sval_t
builtin_head(struct env *env, sval_t args)
{
	assert(env != NULL);

	sval_t l = car(args);
	if (!is_nonempty_list(l))
		return err_undef(); /* TODO: err_empty_list */

	return car(l);
}

static sval_t
builtin_tail(struct env *env, sval_t args)
{
	assert(env != NULL);

	sval_t l = eval(env, car(args));
	if (!is_nonempty_list(l))
		return err_undef(); /* TODO: err_empty_list */

	return cdr(l);
}

void
builtin_init(void)
{
	builtin.sym.def		= sym("def");

	builtin.sym.head	= sym("head");
	builtin.lambda.head	= lambda_builtin(builtin_head, 1);

	builtin.sym.quote	= sym("quote");

	builtin.sym.tail	= sym("tail");
	builtin.lambda.tail	= lambda_builtin(builtin_tail, 1);

	builtin_init_calls++;
}

void
builtin_free(void)
{
	if (0 < builtin_init_calls)
		builtin_init_calls--;

	lambda_free_builtin(builtin.lambda.head);
	lambda_free_builtin(builtin.lambda.tail);
}
