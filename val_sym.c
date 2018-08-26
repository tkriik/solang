#include <assert.h>
#include <string.h>

#include "val.h"

/*
 * TODO: Use global hash table for avoiding allocating duplicate symbols
 */

val_t
mk_sym(const char *s, size_t len)
{
	assert(*s != '\0');
	assert(0 < len);

	val_t v = _mk_undef();
	char *sym = strndup(s, len);

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
