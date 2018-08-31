#ifndef BUILTIN_H
#define BUILTIN_H

#include "val.h"

struct builtin_entry {
	val_t		sym;
	builtin_fn	fn;
};

struct {
	struct builtin_entry quote;
} BUILTIN;

void builtin_init(void);

#endif
