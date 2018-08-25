#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "sym.h"
#include "val.h"

val_t
mk_sym(const char *s, size_t len)
{
	sym_t sym = sym_alloc(s, len);

	val_t v;
	v.p = sym;
	v.u |= (VAL_BOXED_TYPE_SYM << VAL_BOXED_TYPE_OFFSET);
	v.u |= (VAL_STORAGE_BOXED << VAL_STORAGE_OFFSET);

	return v;
}

static unsigned long
get_storage(val_t v)
{
	return (v.u & VAL_STORAGE_MASK) >> VAL_STORAGE_OFFSET;
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

int
is_sym(val_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED
	    && get_boxed_type(v) == VAL_BOXED_TYPE_SYM;
}

const char *
get_sym_str(val_t v)
{
	assert(is_sym(v));

	return sym_str(get_boxed_ptr(v));
}

void
val_debug(val_t v)
{
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
		break;
	default:
		printf("UNKNOWN (%lu)\n", storage);
		break;
	}
}
