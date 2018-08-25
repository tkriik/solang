#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "sym.h"
#include "val.h"

val_t
mk_null(void)
{
	val_t v;
	v.u = 0;
	v.u |= VAL_IMMED_TYPE_NULL << VAL_IMMED_TYPE_OFFSET;
	v.u |= VAL_STORAGE_IMMED << VAL_STORAGE_OFFSET;

	return v;
}

val_t
mk_sym(const char *s, size_t len)
{
	sym_t sym = sym_alloc(s, len);

	val_t v;
	v.p = sym;
	v.u |= VAL_BOXED_TYPE_SYM << VAL_BOXED_TYPE_OFFSET;
	v.u |= VAL_STORAGE_BOXED << VAL_STORAGE_OFFSET;

	return v;
}

static unsigned long
get_storage(val_t v)
{
	return (v.u & VAL_STORAGE_MASK) >> VAL_STORAGE_OFFSET;
}

static unsigned long
get_immed_type(val_t v)
{
	assert(get_storage(v) == VAL_STORAGE_IMMED);

	return (v.u & VAL_IMMED_TYPE_MASK) >> VAL_IMMED_TYPE_OFFSET;
}

static unsigned long
get_boxed_type(val_t v)
{
	assert(get_storage(v) == VAL_STORAGE_BOXED);

	return (v.u & VAL_BOXED_TYPE_MASK) >> VAL_BOXED_TYPE_OFFSET;
}

static void *
get_boxed_ptr(val_t v)
{
	v.u &= VAL_BOXED_MASK;
	return v.p;
}

static int
is_immed(val_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED;
}

static int
is_boxed(val_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED;
}

int
is_null(val_t v)
{
	return get_storage(v) == VAL_STORAGE_IMMED
	    && get_immed_type(v) == VAL_IMMED_TYPE_NULL;
}

int
is_sym(val_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED
	    && get_boxed_type(v) == VAL_BOXED_TYPE_SYM;
}

int
is_eq(val_t v, val_t w)
{
	unsigned long v_storage = get_storage(v);

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

const char *
get_sym_str(val_t v)
{
	assert(is_sym(v));

	return sym_str(get_boxed_ptr(v));
}

void
val_free(val_t v)
{
	switch (get_storage(v)) {
	case VAL_STORAGE_IMMED:
		break;
	case VAL_STORAGE_BOXED:
		switch (get_boxed_type(v)) {
		case VAL_BOXED_TYPE_SYM:
			sym_free(get_boxed_ptr(v));
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

void
val_debug(const char *info, val_t v)
{
	printf("-------- %s\n", info);

	printf("decimal:\t%lu\n", v.u);

	if (sizeof(val_t) == 8)
		printf("hexadecimal:\t0x%016lx\n", v.u);
	else
		printf("hexadecimal:\t0x%08lx\n", v.u);

	printf("bits:\t\t");
	for (size_t i = 0; i < VAL_BITS; i++) {
		unsigned long b = v.u & (1UL << (VAL_BITS - i - 1));
		printf("%s", b ? "1" : "0");
		if (i != 0 && i % 8 == 0)
			printf(" ");
	}
	printf("\n");

	printf("storage:\t");
	unsigned long storage = get_storage(v);
	switch (storage) {
	case VAL_STORAGE_BOXED:
		printf("boxed (%lu)\n", storage);
		printf("pointer:\t%p\n", get_boxed_ptr(v));
		printf("boxed type:\t");

		unsigned long boxed_type = get_boxed_type(v);
		switch (boxed_type) {
		case VAL_BOXED_TYPE_SYM:
			printf("symbol (%lu)\n", boxed_type);
			printf("symbol value:\t\"%s\"\n", get_sym_str(v));
			break;
		default:
			printf("UNKNOWN (%lu)\n", boxed_type);
			break;
		}
		break;

	case VAL_STORAGE_IMMED:
		printf("immediate (%lu)\n", storage);
		printf("immediate type:\t");

		unsigned long immed_type = get_immed_type(v);
		switch (immed_type) {
		case VAL_IMMED_TYPE_NULL:
			printf("null (%lu)\n", immed_type);
			break;
		default:
			printf("UNKNOWN (%lu)\n", immed_type);
			break;
		}
		break;

	default:
		printf("UNKNOWN (%lu)\n", storage);
		break;
	}

	printf("--------\n");
}
