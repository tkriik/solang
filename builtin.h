#ifndef BUILTIN_H
#define BUILTIN_H

#include "sval.h"

struct builtin_entry {
	sval_t sym;
	sval_t lambda;
};

struct {
	struct {
		sval_t def;
		sval_t quote;
	} sym;

	struct {
		sval_t quote;
	} lambda;
} builtin;

void builtin_init(void);
void builtin_free(void);

#endif
