#include <assert.h>

#include "sval.h"

unsigned long
get_storage(sval_t v)
{
	return (v.u & VAL_STORAGE_MASK) >> VAL_STORAGE_OFFSET;
}

unsigned long
get_immed_type(sval_t v)
{
	assert(is_immed(v));

	return (v.u & VAL_IMMED_TYPE_MASK) >> VAL_IMMED_TYPE_OFFSET;
}

unsigned long
get_immed(sval_t v)
{
	assert(is_immed(v));

	return (v.u & VAL_IMMED_MASK) >> VAL_IMMED_OFFSET;
}

void
set_immedempty_list(sval_t *vp)
{
	assert(is_err_undef(*vp));

	vp->u |= VAL_STORAGE_IMMED << VAL_STORAGE_OFFSET;
	vp->u |= VAL_IMMED_TYPE_ELIST << VAL_IMMED_TYPE_OFFSET;
}

unsigned long
get_boxed_type(sval_t v)
{
	assert(is_boxed(v));

	return (v.u & VAL_BOXED_TYPE_MASK) >> VAL_BOXED_TYPE_OFFSET;
}

void *
get_boxed_ptr(sval_t v)
{
	assert(is_boxed(v));

	v.u &= VAL_BOXED_MASK;

	return v.p;
}

void *
get_boxed_sym_ptr(sval_t v)
{
	assert(is_sym(v));
	void *p = get_boxed_ptr(v);

	assert(p != NULL);
	return p;
}

void
set_boxed_sym(sval_t *vp, void *p)
{
	assert(is_err_undef(*vp));
	assert(p != NULL);

	vp->p = p;
	vp->u |= VAL_BOXED_TYPE_SYM << VAL_BOXED_TYPE_OFFSET;
	vp->u |= VAL_STORAGE_BOXED << VAL_STORAGE_OFFSET;
}

void *
get_boxed_list_ptr(sval_t v)
{
	assert(is_boxed(v));

	void *p = get_boxed_ptr(v);

	assert(p != NULL);
	return p;
}

void
set_boxed_list(sval_t *vp, void *p)
{
	assert(is_err_undef(*vp));
	assert(p != NULL);

	vp->p = p;
	vp->u |= VAL_STORAGE_BOXED << VAL_STORAGE_OFFSET;
	vp->u |= VAL_BOXED_TYPE_LIST << VAL_BOXED_TYPE_OFFSET;
}

void *
get_boxed_lambda_ptr(sval_t v)
{
	assert(is_boxed(v));

	void *p = get_boxed_ptr(v);

	assert(p != NULL);
	return p;
}

void
set_boxed_lambda(sval_t *vp, void *p)
{
	assert(is_err_undef(*vp));
	assert(p != NULL);

	vp->p = p;
	vp->u |= VAL_STORAGE_BOXED << VAL_STORAGE_OFFSET;
	vp->u |= VAL_BOXED_TYPE_LAMBDA << VAL_BOXED_TYPE_OFFSET;
}
