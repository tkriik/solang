#include <assert.h>

#include "builtin.h"
#include "env.h"
#include "eval.h"
#include "val.h"

static int
is_self_eval(val_t exp)
{
	// TODO
	return 0;
}

static int
is_def(val_t exp)
{
	return is_triple(exp)
	    && is_eq(car(exp), builtin.sym.def)
	    && is_sym(car(cdr(exp)));
}

static int
is_application(val_t exp)
{
	return is_nonempty_list(exp) && is_sym(car(exp));
}

static val_t
do_def(struct env *env, val_t exp)
{
	assert(env != NULL);
	assert(is_def(exp));

	val_t sym = car(cdr(exp));
	val_t v = eval(env, car(cdr(cdr(exp))));

	if (is_eq(sym, v))
		return v;

	return env_define(env, sym, v);
}

static val_t
do_apply(struct env *env, val_t exp)
{
	assert(env != NULL);
	assert(is_application(exp));

	val_t sym = car(exp);
	val_t lambda = env_lookup(env, sym);
	if (is_err_undef(lambda))
		return err_undef(); /* TODO: err_no_sym */

	val_t args = cdr(exp);

	return lambda_apply(env, lambda, args);
}

val_t
eval(struct env *env, val_t exp)
{
	assert(env != NULL);

	if (is_self_eval(exp))
		return exp;

	if (is_sym(exp))
		return env_lookup(env, exp);

	if (is_quoted(exp))
		return unquote(exp);

	if (is_def(exp))
		return do_def(env, exp);

	if (is_application(exp))
		return do_apply(env, exp);

	return err_undef();
}
