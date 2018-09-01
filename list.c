#include <assert.h>
#include <stdlib.h>

#include "sval.h"

struct blist {
	sval_t hd;
	sval_t tl;
};

sval_t
list(void)
{
	sval_t v = err_undef();
	set_immedempty_list(&v);

	return v;
}

sval_t
nonempty_list(sval_t hd, sval_t tl)
{
	assert(is_list(tl));

	struct blist *bl = calloc(1, sizeof(*bl));
	assert(bl != NULL);

	bl->hd = hd;
	bl->tl = tl;

	sval_t v = err_undef();
	set_boxed_list(&v, bl);

	return v;
}

int
is_empty_list(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED
	    && get_immed_type(v) == VAL_IMMED_TYPE_ELIST;
}

int
is_nonempty_list(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED
	    && get_boxed_type(v) == VAL_BOXED_TYPE_LIST;
}

int
is_list(sval_t v)
{
	return is_nonempty_list(v) || is_empty_list(v);
}

int
is_pair(sval_t v)
{
	return is_nonempty_list(v)
	    && is_nonempty_list(cdr(v))
	    && is_empty_list(cdr(cdr(v)));
}

int
is_triple(sval_t v)
{
	return is_nonempty_list(v)
	    && is_nonempty_list(cdr(v))
	    && is_nonempty_list(cdr(cdr(v)))
	    && is_empty_list(cdr(cdr(cdr(v))));
}

sval_t
cons(sval_t v, sval_t l)
{
	assert(is_list(l));

	return nonempty_list(v, l);
}

sval_t
car(sval_t l)
{
	struct blist *bl = get_boxed_list_ptr(l);

	return bl->hd;
}

sval_t
cdr(sval_t l)
{
	struct blist *bl = get_boxed_list_ptr(l);

	assert(is_list(bl->tl));
	return bl->tl;
}

size_t
list_count(sval_t l)
{
	assert(is_list(l));

	size_t count = 0;
	sval_t node = l;
	while (is_nonempty_list(node)) {
		count++;
		node = cdr(node);
	}

	return count;
}

static sval_t
blist_reverse_inplace(sval_t l)
{
	sval_t p = l;
	sval_t q = list();
	sval_t r;

	while (!is_empty_list(p)) {
		r = q;
		q = p;
		p = cdr(p);
		struct blist *bl_q = get_boxed_list_ptr(q);
		bl_q->tl = r;
	}

	return q;
}

sval_t
list_reverse_inplace(sval_t l)
{
	assert(is_list(l));

	if (is_empty_list(l))
		return l;

	return blist_reverse_inplace(l);
}

int
nonempty_list_eq(sval_t l0, sval_t l1)
{
	assert(is_nonempty_list(l0));
	assert(is_nonempty_list(l1));

	sval_t node0 = l0;
	sval_t node1 = l1;

	while (1) {
		if (is_empty_list(node0) && is_empty_list(node1))
			return 1;

		if (!is_nonempty_list(node0) || !is_nonempty_list(node1))
			return 0;

		sval_t hd0 = car(node0);
		sval_t hd1 = car(node1);
		if (!is_eq(hd0, hd1))
			return 0;

		node0 = cdr(node0);
		node1 = cdr(node1);
	}
}

void
nonempty_list_free(sval_t l)
{
	assert(is_nonempty_list(l));

	sval_t node = l;
	while (!is_empty_list(node)) {
		sval_t tmp = cdr(node);
		sval_free(car(node));
		free(get_boxed_list_ptr(node));
		node = tmp;
	}
}
