#ifndef ENV_H
#define ENV_H

#include "uthash.h"

#include "val.h"

struct env_entry;

struct env {
	struct env_entry *entries;
};

void	env_init(struct env *);
void	env_destroy(struct env *);

val_t	env_define(struct env *, val_t, val_t);
val_t	env_lookup(struct env *, val_t);

#endif
