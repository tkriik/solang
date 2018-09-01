#include <assert.h>
#include <stdlib.h>

#include "val.h"

struct blist {
	val_t hd;
	val_t tl;
};

val_t
list(void)
{
	val_t v = err_undef();
	set_immedempty_list(&v);

	return v;
}

val_t
nonempty_list(val_t hd, val_t tl)
{
	assert(is_list(tl));

	struct blist *bl = calloc(1, sizeof(*bl));
	assert(bl != NULL);

	bl->hd = hd;
	bl->tl = tl;

	val_t v = err_undef();
	set_boxed_list(&v, bl);

	return v;
}

int
is_empty_list(val_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED
	    && get_immed_type(v) == VAL_IMMED_TYPE_ELIST;
}

int
is_nonempty_list(val_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED
	    && get_boxed_type(v) == VAL_BOXED_TYPE_LIST;
}

int
is_list(val_t v)
{
	return is_nonempty_list(v) || is_empty_list(v);
}

int
is_pair(val_t v)
{
	return is_nonempty_list(v)
	    && is_nonempty_list(cdr(v))
	    && is_empty_list(cdr(cdr(v)));
}

int
is_triple(val_t v)
{
	return is_nonempty_list(v)
	    && is_nonempty_list(cdr(v))
	    && is_nonempty_list(cdr(cdr(v)))
	    && is_empty_list(cdr(cdr(cdr(v))));
}

val_t
cons(val_t v, val_t l)
{
	assert(is_list(l));

	return nonempty_list(v, l);
}

val_t
car(val_t l)
{
	struct blist *bl = get_boxed_list_ptr(l);

	return bl->hd;
}

val_t
cdr(val_t l)
{
	struct blist *bl = get_boxed_list_ptr(l);

	assert(is_list(bl->tl));
	return bl->tl;
}

size_t
list_count(val_t l)
{
	assert(is_list(l));

	size_t count = 0;
	val_t node = l;
	while (is_nonempty_list(node)) {
		count++;
		node = cdr(node);
	}

	return count;
}

static val_t
blist_reverse_inplace(val_t l)
{
	val_t p = l;
	val_t q = list();
	val_t r;

	while (!is_empty_list(p)) {
		r = q;
		q = p;
		p = cdr(p);
		struct blist *bl_q = get_boxed_list_ptr(q);
		bl_q->tl = r;
	}

	return q;
}

val_t
list_reverse_inplace(val_t l)
{
	assert(is_list(l));

	if (is_empty_list(l))
		return l;

	return blist_reverse_inplace(l);
}

int
nonempty_list_eq(val_t l0, val_t l1)
{
	assert(is_nonempty_list(l0));
	assert(is_nonempty_list(l1));

	val_t node0 = l0;
	val_t node1 = l1;

	while (1) {
		if (is_empty_list(node0) && is_empty_list(node1))
			return 1;

		if (!is_nonempty_list(node0) || !is_nonempty_list(node1))
			return 0;

		val_t hd0 = car(node0);
		val_t hd1 = car(node1);
		if (!is_eq(hd0, hd1))
			return 0;

		node0 = cdr(node0);
		node1 = cdr(node1);
	}
}

void
nonempty_list_free(val_t l)
{
	assert(is_nonempty_list(l));

	val_t node = l;
	while (!is_empty_list(node)) {
		val_t tmp = cdr(node);
		val_free(car(node));
		free(get_boxed_list_ptr(node));
		node = tmp;
	}
}
