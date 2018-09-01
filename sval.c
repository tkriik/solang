#include <assert.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "builtin.h"
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
set_immed_empty_list(sval_t *vp)
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

int
is_immed(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED;
}

int
is_boxed(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED;
}

sval_t
quote(sval_t v)
{
	return nonempty_list(builtin.sym.quote, nonempty_list(v, list()));
}

sval_t
unquote(sval_t v)
{
	assert(is_quoted(v));

	return car(cdr(v));
}

int
is_quoted(sval_t v)
{
	return is_nonempty_list(v) && is_eq(builtin.sym.quote, car(v));
}

int
is_eq(sval_t v, sval_t w)
{
	unsigned long v_storage = get_storage(v);

	unsigned long v_immed_type;
	unsigned long w_immed_type;

	unsigned long v_boxed_type;
	unsigned long w_boxed_type;

	const char *v_sym_name;
	const char *w_sym_name;

	switch (v_storage) {
	case VAL_STORAGE_IMMED:
		if (!is_immed(w))
			return 0;

		v_immed_type = get_immed_type(v);
		w_immed_type = get_immed_type(w);
		if (v_immed_type != w_immed_type)
			return 0;

		return 1;

	case VAL_STORAGE_BOXED:
		if (!is_boxed(w))
			return 0;

		v_boxed_type = get_boxed_type(v);
		w_boxed_type = get_boxed_type(w);
		if (v_boxed_type != w_boxed_type)
			return 0;

		switch (v_boxed_type) {
		case VAL_BOXED_TYPE_SYM:
			v_sym_name = sym_name(v);
			w_sym_name = sym_name(w);
			return v.u == w.u
			    && strcmp(v_sym_name, w_sym_name) == 0;

		case VAL_BOXED_TYPE_LIST:
			return nonempty_list_eq(v, w);

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
sval_free(sval_t v)
{
	switch (get_storage(v)) {
	case VAL_STORAGE_IMMED:
		return;

	case VAL_STORAGE_BOXED:
		switch (get_boxed_type(v)) {
		case VAL_BOXED_TYPE_SYM:
			return;

		case VAL_BOXED_TYPE_LIST:
			nonempty_list_free(v);
			return;

		case VAL_BOXED_TYPE_LAMBDA:
			lambda_free(v);
			return;
		}
	}

	assert(0 && "NOTREACHED");
}
