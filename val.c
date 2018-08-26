#include <assert.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "val.h"

val_t
_mk_undef(void)
{
	val_t v;
	v.u = 0;

	return v;
}

val_t
mk_null(void)
{
	val_t v = _mk_undef();

	_set_immed_null(&v);

	return v;
}

int
is_immed(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_IMMED;
}

int
is_boxed(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_BOXED;
}

int
is_null(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_IMMED
	    && _get_immed_type(v) == VAL_IMMED_TYPE_NULL;
}

int
is_eq(val_t v, val_t w)
{
	unsigned long v_storage = _get_storage(v);

	unsigned long v_immed_type;
	unsigned long w_immed_type;

	unsigned long v_boxed_type;
	unsigned long w_boxed_type;

	const char *v_sym_str;
	const char *w_sym_str;

	switch (v_storage) {
	case VAL_STORAGE_IMMED:
		if (!is_immed(w))
			return 0;

		v_immed_type = _get_immed_type(v);
		w_immed_type = _get_immed_type(w);
		if (v_immed_type != w_immed_type)
			return 0;

		return 1;

	case VAL_STORAGE_BOXED:
		if (!is_boxed(w))
			return 0;

		v_boxed_type = _get_boxed_type(v);
		w_boxed_type = _get_boxed_type(w);
		if (v_boxed_type != w_boxed_type)
			return 0;

		switch (v_boxed_type) {
		case VAL_BOXED_TYPE_SYM:
			v_sym_str = get_sym_str(v);
			w_sym_str = get_sym_str(w);
			return strcmp(v_sym_str, w_sym_str) == 0;

		default:
			break;
		}

	default:
		break;
	}

	assert(0 && "NOTREACHED");

	return 0;
}

void
val_free(val_t v)
{
	switch (_get_storage(v)) {
	case VAL_STORAGE_IMMED:
		break;
	case VAL_STORAGE_BOXED:
		switch (_get_boxed_type(v)) {
		case VAL_BOXED_TYPE_SYM:
			free(_get_boxed_sym_ptr(v));
			return;
		default:
			assert(0 && "NOTREACHED");
			return;
		}
	default:
		assert(0 && "NOTREACHED");
		return;
	}
}
