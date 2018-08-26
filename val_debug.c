#include <stdio.h>

#include "val.h"

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
	unsigned long storage = _get_storage(v);
	switch (storage) {
	case VAL_STORAGE_IMMED:
		printf("immediate (%lu)\n", storage);
		printf("immediate type:\t");

		unsigned long immed_type = _get_immed_type(v);
		switch (immed_type) {
		case VAL_IMMED_TYPE_UNDEF:
			printf("undefined (%lu)\n", immed_type);
			break;
		case VAL_IMMED_TYPE_NULL:
			printf("null (%lu)\n", immed_type);
			break;
		default:
			printf("UNKNOWN (%lu)\n", immed_type);
			break;
		}
		break;

	case VAL_STORAGE_BOXED:
		printf("boxed (%lu)\n", storage);
		printf("pointer:\t%p\n", _get_boxed_ptr(v));
		printf("boxed type:\t");

		unsigned long boxed_type = _get_boxed_type(v);
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

	default:
		printf("UNKNOWN (%lu)\n", storage);
		break;
	}

	printf("--------\n");
}
