#include <assert.h>

#include "env.h"
#include "val.h"
#include "vtab.h"

void
env_init(struct env *env)
{
	assert(env != NULL);

	struct vtab *vtab = &env->vtab;

	vtab_init(vtab);

	/* TODO: store builtins elsewhere */
	vtab_insert(vtab, sym("def", 3), nil());
}

val_t
env_define(struct env *env, val_t sym, val_t v)
{
	assert(env != NULL);
	assert(is_sym(sym));
	assert(!_is_undef(v));

	return vtab_insert(&env->vtab, sym, v);
}

val_t
env_lookup(struct env *env, val_t sym)
{
	assert(env != NULL);
	assert(is_sym(sym));

	return vtab_lookup(&env->vtab, sym);
}
