#include <assert.h>

#include "val.h"

void
assert_undef(val_t v)
{
	assert(v.u == 0);
}

void
assert_immed(val_t v)
{
	assert(_get_storage(v) == VAL_STORAGE_IMMED);
}

void
assert_boxed(val_t v)
{
	assert(_get_storage(v) == VAL_STORAGE_BOXED);
}

void
assert_boxed_sym(val_t v)
{
	assert(_get_boxed_type(v) == VAL_BOXED_TYPE_SYM);
}
