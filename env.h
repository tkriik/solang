#ifndef SOLANG_ENV_H
#define SOLANG_ENV_H

#include "uthash.h"

#include "sval.h"

struct env_entry;

struct env {
	struct env_entry *entries;
};

void	env_init(struct env *);
void	env_destroy(struct env *);

sval_t	env_define(struct env *, sval_t, sval_t);
sval_t	env_lookup(struct env *, sval_t);

#endif
