#ifndef ENV_H
#define ENV_H

#include "val.h"
#include "vtab.h"

struct env {
	struct vtab vtab;
};

void	env_init(struct env *);

val_t	env_define(struct env *, val_t, val_t);
val_t	env_lookup(struct env *, val_t);

#endif
