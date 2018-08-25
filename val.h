#ifndef VAL_H
#define VAL_H

/*
 * A tal value is a 64-bit or 32-bit word (depending on the architecture),
 * which either contains an immediate value (stored in the word) or a pointer
 * to the heap (in which case the value is boxed).
 *
 * The first LSB (least significant bit) is a storage tag, and it denotes
 * whether the value is immediate or boxed:
 *   - 0: immediate
 *   - 1: boxed
 *
 * The next LSB after the storage tag denotes the type of the value.
 *
 *   For boxed values, the types are:
 *     - 0: symbol
 *     - 1: <TODO>
 *
 *   For immediate values, the types are:
 *     - 0: <TODO>
 *     - 1: <TODO>
 *
 * Value info:
 *
 *   Symbol:
 *     - A symbol contains a pointer to a heap-allocated string.
 */

#include <stddef.h>
#include <limits.h>

typedef union {
	unsigned long	 u;
	void		*p;
} val_t;

enum val_storage {
	VAL_STORAGE_IMMED	= 0,
	VAL_STORAGE_BOXED	= 1
};

enum val_boxed_type {
	VAL_BOXED_TYPE_SYM	= 0
};

enum val_bits {
	VAL_BITS		= sizeof(val_t) * 8,
	VAL_STORAGE_BITS	= 1,
	VAL_BOXED_TYPE_BITS	= 1,
	VAL_BOXED_BITS		= VAL_BITS - (VAL_STORAGE_BITS + VAL_BOXED_TYPE_BITS)
};

enum val_offset {
	VAL_STORAGE_OFFSET	= 0,
	VAL_BOXED_TYPE_OFFSET	= VAL_STORAGE_OFFSET + VAL_STORAGE_BITS,
	VAL_BOXED_OFFSET	= VAL_BOXED_TYPE_OFFSET + VAL_BOXED_TYPE_BITS
};

enum val_mask {
	VAL_STORAGE_MASK	= ((1 << VAL_STORAGE_BITS) - 1) << VAL_STORAGE_OFFSET,
	VAL_BOXED_TYPE_MASK	= ((1 << VAL_BOXED_TYPE_BITS) - 1) << VAL_BOXED_TYPE_OFFSET,
	VAL_BOXED_MASK		= (unsigned long)~0 << VAL_BOXED_OFFSET
};

enum val_lim {
	VAL_BOXED_LIM		= ULONG_MAX >> VAL_BOXED_OFFSET
};

val_t		 mk_sym(const char *, size_t);
int		 is_sym(val_t);
const char	*get_sym_str(val_t);

void		 val_debug(val_t);

#endif
