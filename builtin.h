#ifndef BUILTIN_H
#define BUILTIN_H

#include "val.h"

struct {
	struct {
		val_t quote;
	} sym;
} BUILTIN;

void builtin_init(void);

#endif
