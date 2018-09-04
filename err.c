#include <assert.h>

#include "sval.h"

sval_t
err_undef(void)
{
	sval_t v;

	v.u = 0;
	v.u |= VAL_STORAGE_IMMED << VAL_STORAGE_OFFSET;
	v.u |= VAL_IMMED_TYPE_ERR << VAL_IMMED_TYPE_OFFSET;
	v.u |= VAL_IMMED_ERR_UNDEF << VAL_IMMED_OFFSET;

	return v;
}

int
is_err_undef(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED
	    && get_immed_type(v) == VAL_IMMED_TYPE_ERR;
}

int
is_err(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED
	    && get_immed_type(v) == VAL_IMMED_TYPE_ERR
	    && get_immed(v) == VAL_IMMED_ERR_UNDEF;
}

const char *
err_str(sval_t v)
{
	assert(is_err(v));

	switch (get_immed(v)) {
	case VAL_IMMED_ERR_UNDEF:	return "#error<undefined>";
	case VAL_IMMED_ERR_NOMEM:	return "#error<out-of-memory>";
	}

	assert(0 && "NOTREACHED");
}
