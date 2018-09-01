#ifndef BUILTIN_H
#define BUILTIN_H

#include "val.h"

struct builtin_entry {
	val_t sym;
	val_t lambda;
};

struct {
	struct {
		val_t def;
		val_t quote;
	} sym;

	struct {
		val_t quote;
	} lambda;
} builtin;

void builtin_init(void);
void builtin_free(void);

#endif
