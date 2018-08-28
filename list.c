#include <assert.h>
#include <stdlib.h>

#include "val.h"

struct blist {
	val_t hd;
	val_t tl;
};

val_t
_elist(void)
{
	val_t v = _undef();
	_set_immed_elist(&v);

	return v;
}

val_t
_blist(val_t hd, val_t tl)
{
	assert_list(tl);

	struct blist *bl = calloc(1, sizeof(*bl));
	assert(bl != NULL);

	bl->hd = hd;
	bl->tl = tl;

	val_t v = _undef();
	_set_boxed_list(&v, bl);

	return v;
}

val_t
list(void)
{
	return _elist();
}

int
_is_elist(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_IMMED
	    && _get_immed_type(v) == VAL_IMMED_TYPE_ELIST;
}

int
_is_blist(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_BOXED
	    && _get_boxed_type(v) == VAL_BOXED_TYPE_LIST;
}

int
is_list(val_t v)
{
	return _is_blist(v) || _is_elist(v);
}

val_t
cons(val_t v, val_t l)
{
	assert_list(l);

	return _blist(v, l);
}

val_t
list_head(val_t l)
{
	struct blist *bl = _get_boxed_list_ptr(l);

	return bl->hd;
}

val_t
list_tail(val_t l)
{
	struct blist *bl = _get_boxed_list_ptr(l);

	assert_list(bl->tl);
	return bl->tl;
}

size_t
list_count(val_t l)
{
	assert_list(l);

	size_t count = 0;
	val_t node = l;
	while (_is_blist(node)) {
		count++;
		node = list_tail(node);
	}

	return count;
}

static val_t
blist_reverse_inplace(val_t l)
{
	val_t p = l;
	val_t q = list();
	val_t r;

	while (!_is_elist(p)) {
		r = q;
		q = p;
		p = list_tail(p);
		struct blist *bl_q = _get_boxed_list_ptr(q);
		bl_q->tl = r;
	}

	return q;
}

val_t
list_reverse_inplace(val_t l)
{
	assert_list(l);

	if (_is_elist(l))
		return l;

	return blist_reverse_inplace(l);
}

int
_blist_eq(val_t l0, val_t l1)
{
	_assert_boxed_list(l0);
	_assert_boxed_list(l1);

	val_t node0 = l0;
	val_t node1 = l1;

	while (1) {
		if (_is_elist(node0) && _is_elist(node1))
			return 1;

		if (!_is_blist(node0) || !_is_blist(node1))
			return 0;

		val_t hd0 = list_head(node0);
		val_t hd1 = list_head(node1);
		if (!is_eq(hd0, hd1))
			return 0;

		node0 = list_tail(node0);
		node1 = list_tail(node1);
	}
}

void
_blist_free(val_t l)
{
	_assert_boxed_list(l);

	val_t node = l;
	while (!_is_elist(node)) {
		val_t tmp = list_tail(node);
		val_free(list_head(node));
		free(_get_boxed_list_ptr(node));
		node = tmp;
	}
}
