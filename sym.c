#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "val.h"

/*
 * TODO: Use global hash table for avoiding allocating duplicate symbols
 */

val_t
sym(const char *s, size_t len)
{
	assert(*s != '\0');
	assert(0 < len);
	assert(len < SIZE_MAX);

	val_t v = _undef();

	char *sym = calloc(1, len + 1);
	assert(sym != NULL);
	memcpy(sym, s, len);

	_set_boxed_sym(&v, sym);

	return v;
}

int
is_sym(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_BOXED
	    && _get_boxed_type(v) == VAL_BOXED_TYPE_SYM;
}

const char *
get_sym_str(val_t v)
{
	return (const char*)(_get_boxed_sym_ptr(v));
}
