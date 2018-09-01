#include <assert.h>

#include "builtin.h"
#include "env.h"
#include "sval.h"

struct env_entry {
	unsigned long	sym_u;
	sval_t		v;

	UT_hash_handle	hh;
};

void
env_init(struct env *env)
{
	assert(env != NULL);

	builtin_init();

	env->entries = NULL;

	sval_t res = env_define(env, builtin.sym.quote, builtin.lambda.quote);
	assert(is_sym(res));
}

void
env_destroy(struct env *env)
{
	assert(env != NULL);

	struct env_entry *entry, *tmp;
	HASH_ITER(hh, env->entries, entry, tmp) {
		HASH_DEL(env->entries, entry);
		sval_free(entry->v);
		free(entry);
	}

	HASH_CLEAR(hh, env->entries);

	builtin_free();
}

sval_t
env_define(struct env *env, sval_t sym, sval_t v)
{
	assert(env != NULL);
	assert(is_sym(sym));
	assert(!is_err_undef(v)); /* TODO: err_nodef */

	struct env_entry *entry;
	HASH_FIND(hh, env->entries, &sym.u, sizeof(sym.u), entry);
	if (entry != NULL)
		return err_undef(); /* TODO: err_redefine */

	entry = malloc(sizeof(*entry));
	assert(entry != NULL);
	entry->sym_u = sym.u;
	entry->v = v;

	HASH_ADD(hh, env->entries, sym_u, sizeof(sym.u), entry);

	return sym;
}

sval_t
env_lookup(struct env *env, sval_t sym)
{
	assert(env != NULL);
	assert(is_sym(sym));

	struct env_entry *entry;
	HASH_FIND(hh, env->entries, &sym.u, sizeof(sym.u), entry);
	if (entry == NULL)
		return err_undef();

	return entry->v;
}
