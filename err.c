#include <assert.h>

#include "val.h"

val_t
err_undef(void)
{
	val_t v;

	v.u = 0;
	v.u |= VAL_STORAGE_IMMED << VAL_STORAGE_OFFSET;
	v.u |= VAL_IMMED_TYPE_ERR << VAL_IMMED_TYPE_OFFSET;
	v.u |= VAL_IMMED_ERR_UNDEF << VAL_IMMED_OFFSET;

	return v;
}

int
is_err_undef(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_IMMED
	    && _get_immed_type(v) == VAL_IMMED_TYPE_ERR;
}

int
is_err(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_IMMED
	    && _get_immed_type(v) == VAL_IMMED_TYPE_ERR
	    && _get_immed(v) == VAL_IMMED_ERR_UNDEF;
}

const char *
err_str(val_t v)
{
	assert(is_err(v));

	switch (_get_immed(v)) {
	case VAL_IMMED_ERR_UNDEF:	return "undefined";
	case VAL_IMMED_ERR_NOMEM:	return "out-of-memory";
	}

	assert(0 && "NOTREACHED");
}
