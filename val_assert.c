#include <assert.h>

#include "val.h"

/*
 * TODO: value integrity checks
 */

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
_assert_immed_elist(val_t v)
{
	assert(_get_immed_type(v) == VAL_IMMED_TYPE_ELIST);
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

void
_assert_boxed_list(val_t v)
{
	assert(_get_boxed_type(v) == VAL_BOXED_TYPE_LIST);
}

void
assert_list(val_t v)
{
	assert(is_list(v));
}
