#include "env.h"
#include "eval.h"
#include "val.h"

int
is_self_eval(val_t exp)
{
	// TODO
	return 0;
}

val_t
eval(struct env *env, val_t exp)
{
	if (is_self_eval(exp))
		return exp;

	if (is_sym(exp))
		return env_lookup(env, exp);

	return _undef();
}
