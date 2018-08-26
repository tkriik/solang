#include "val.h"

unsigned long
_get_storage(val_t v)
{
	return (v.u & VAL_STORAGE_MASK) >> VAL_STORAGE_OFFSET;
}

unsigned long
_get_immed_type(val_t v)
{
	assert_immed(v);

	return (v.u & VAL_IMMED_TYPE_MASK) >> VAL_IMMED_TYPE_OFFSET;
}

void
_set_immed_null(val_t *vp)
{
	assert_undef(*vp);

	vp->u |= VAL_IMMED_TYPE_NULL << VAL_IMMED_TYPE_OFFSET;
	vp->u |= VAL_STORAGE_IMMED << VAL_STORAGE_OFFSET;
}

unsigned long
_get_boxed_type(val_t v)
{
	assert_boxed(v);

	return (v.u & VAL_BOXED_TYPE_MASK) >> VAL_BOXED_TYPE_OFFSET;
}

void *
_get_boxed_ptr(val_t v)
{
	assert_boxed(v);

	v.u &= VAL_BOXED_MASK;

	return v.p;
}

void *
_get_boxed_sym_ptr(val_t v)
{
	assert_boxed_sym(v);

	return _get_boxed_ptr(v);
}

void
_set_boxed_sym(val_t *vp, void *p)
{
	assert_undef(*vp);

	vp->p = p;
	vp->u |= VAL_BOXED_TYPE_SYM << VAL_BOXED_TYPE_OFFSET;
	vp->u |= VAL_STORAGE_BOXED << VAL_STORAGE_OFFSET;
}
