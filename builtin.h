#ifndef SOLANG_BUILTIN_H
#define SOLANG_BUILTIN_H

#include "sval.h"

struct builtin_entry {
	sval_t sym;
	sval_t lambda;
};

struct builtin_info {
	struct {
		sval_t def;
		sval_t head;
		sval_t quote;
		sval_t tail;
	} sym;

	struct {
		sval_t head;
		sval_t quote;
		sval_t tail;
	} lambda;
};

extern struct builtin_info builtin;

void builtin_init(void);
void builtin_free(void);

#endif
