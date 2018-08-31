#include <assert.h>
#include <stdarg.h>
#include <stdio.h>

#include "val.h"

static int indent_width	= 8;
static int field_width	= 16;

static void
sub_indent(int depth)
{
	assert(2 <= field_width);

	if (depth == 0)
		return;

	for (int i = 0; i < depth; i++) {
		if (i == depth - 1)
			printf("%*s", indent_width - 2, "");
		else
			printf("%*s", indent_width, "");
	}

	printf("> ");
}

static void
depth_vprintf(int depth, const char *field, const char *fmt, va_list args)
{
	sub_indent(depth);
	printf("%-*s ", field_width, field);
	vprintf(fmt, args);
}

static void
depth_printf(int depth, const char *field, const char *fmt, ...)
{
	va_list args;
	va_start(args, fmt);

	depth_vprintf(depth, field, fmt, args);

	va_end(args);
}

static void
do_val_debug(val_t v, int depth)
{
	depth_printf(depth, "decimal", "%lu\n", v.u);

	if (sizeof(val_t) == 8)
		depth_printf(depth, "hexadecimal", "0x%016lx\n", v.u);
	else
		depth_printf(depth, "hexadecimal", "0x%08lx\n", v.u);

	depth_printf(depth, "bits", "");
	for (size_t i = 0; i < VAL_BITS; i++) {
		unsigned long b = v.u & (1UL << (VAL_BITS - i - 1));
		printf("%s", b ? "1" : "0");
		if (i != 0 && i % 8 == 0)
			printf(" ");
	}
	printf("\n");

	depth_printf(depth, "storage", "");
	unsigned long storage = _get_storage(v);
	switch (storage) {
	case VAL_STORAGE_IMMED:
		printf("immediate (%lu)\n", storage);
		depth_printf(depth, "immediate type", "");

		unsigned long immed_type = _get_immed_type(v);
		switch (immed_type) {
		case VAL_IMMED_TYPE_ERR:
			printf("error (%lu)\n", immed_type);
			depth_printf(depth, "error", "%s\n", err_str(v));
			break;
		case VAL_IMMED_TYPE_ELIST:
			printf("empty list (%lu)\n", immed_type);
			break;
		default:
			printf("<INVALID> (%lu)\n", immed_type);
			break;
		}
		break;

	case VAL_STORAGE_BOXED:
		printf("boxed (%lu)\n", storage);
		depth_printf(depth, "pointer", "%p\n", _get_boxed_ptr(v));
		depth_printf(depth, "boxed type", "");

		unsigned long boxed_type = _get_boxed_type(v);
		switch (boxed_type) {
		case VAL_BOXED_TYPE_SYM:
			printf("symbol (%lu)\n", boxed_type);
			depth_printf(depth, "symbol value", "\"%s\"\n", sym_name(v));
			break;
		case VAL_BOXED_TYPE_LIST:
			printf("list (%lu)\n", boxed_type);
			depth_printf(depth, "list count", "%zu\n", list_count(v));
			depth_printf(depth, "list values", "\n");
			val_t node = v;
			while (1) {
				if (_is_elist(node))
					break;
				val_t w = car(node);
				depth_printf(depth + 1, "--------", "\n");
				do_val_debug(w, depth + 1);
				node = cdr(node);
			}
			break;
		default:
			printf("<INVALID> (%lu)\n", boxed_type);
			break;
		}
		break;

	default:
		printf("<INVALID> (%lu)\n", storage);
		break;
	}
}

void
val_debug(const char *info, val_t v)
{
	printf("-------- %s\n", info);
	do_val_debug(v, 0);
	printf("--------\n");
}
