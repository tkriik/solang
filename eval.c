#include <assert.h>

#include "builtin.h"
#include "env.h"
#include "eval.h"
#include "sval.h"

static int
is_self_eval(sval_t exp)
{
	// TODO
	return 0;
}

static int
is_def(sval_t exp)
{
	return is_triple(exp)
	    && is_eq(car(exp), builtin.sym.def)
	    && is_sym(car(cdr(exp)));
}

static int
is_application(sval_t exp)
{
	return is_nonempty_list(exp) && is_sym(car(exp));
}

static sval_t
do_def(struct env *env, sval_t exp)
{
	assert(env != NULL);
	assert(is_def(exp));

	sval_t sym = car(cdr(exp));
	sval_t v = eval(env, car(cdr(cdr(exp))));

	if (is_eq(sym, v))
		return v;

	return env_define(env, sym, v);
}

static sval_t
do_apply(struct env *env, sval_t sym, sval_t args)
{
	assert(env != NULL);
	assert(is_sym(sym));
	assert(is_list(args));

	sval_t lambda = env_lookup(env, sym);
	if (is_err(lambda))
		return err_undef(); /* TODO: err_no_sym */

	return lambda_apply(env, lambda, args);
}

sval_t
eval_args(struct env *env, sval_t args)
{
	sval_t v;
	sval_t l = args;
	sval_t res = list();
	LIST_FOREACH(v, l) {
		sval_t subres = eval(env, v);
		if (is_err(subres)) {
			sval_free(res);
			return subres;
		}

		res = cons(subres, res);
	}

	return list_reverse_inplace(res);
}

sval_t
eval(struct env *env, sval_t exp)
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

	if (is_application(exp)) {
		sval_t sym = car(exp);
		sval_t args = eval_args(env, cdr(exp));
		if (is_err(args))
			return args;

		return do_apply(env, sym, args);
	}

	return err_undef();
}
