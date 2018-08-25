#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "sym.h"

sym_t
sym_alloc(const char *s)
{
	sym_t sym = (sym_t)strdup(s);
	assert(sym != NULL);
	return sym;
}

void
sym_free(sym_t sym)
{
	free(sym);
}

const char *
sym_str(sym_t sym)
{
	return (const char *)sym;
}
