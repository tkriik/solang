#include <assert.h>

#include "env.h"
#include "val.h"

struct env_entry {
	unsigned long	sym_u;
	val_t		v;

	UT_hash_handle	hh;
};

void
env_init(struct env *env)
{
	assert(env != NULL);

	env->entries = NULL;

	/* TODO: define builtins */
}

void
env_destroy(struct env *env)
{
	assert(env != NULL);

	struct env_entry *entry, *tmp;
	HASH_ITER(hh, env->entries, entry, tmp) {
		HASH_DEL(env->entries, entry);
		val_free(entry->v);
	}

	HASH_CLEAR(hh, env->entries);
}

val_t
env_define(struct env *env, val_t sym, val_t v)
{
	assert(env != NULL);
	assert(is_sym(sym));
	assert(!is_err_undef(v));

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

val_t
env_lookup(struct env *env, val_t sym)
{
	assert(env != NULL);
	assert(is_sym(sym));

	struct env_entry *entry;
	HASH_FIND(hh, env->entries, &sym.u, sizeof(sym.u), entry);
	if (entry == NULL)
		return err_undef();

	return entry->v;
}
