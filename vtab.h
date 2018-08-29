#ifndef VTAB_H
#define VTAB_H

#include <stddef.h>

#include "conf.h"
#include "val.h"

struct vtab_entry {
	val_t sym;
	val_t v;
};

struct vtab {
	size_t count;
	struct vtab_entry entries[VTAB_MAX_ENTRIES];
};

void	vtab_init(struct vtab *);

val_t	vtab_insert(struct vtab *, val_t, val_t);
val_t	vtab_lookup(struct vtab *, val_t);

#endif
