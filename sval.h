#ifndef SOLANG_SVAL_H
#define SOLANG_SVAL_H

/*
 * A Solang value is a 64-bit or 32-bit word (depending on the architecture),
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
 *   For immediate values, the types are:
 *     - 0: error
 *     - 1: empty list
 *
 *   For boxed values, the types are:
 *     - 0: error
 *     - 1: symbol
 *     - 2: list
 *     - 3: lambda
 *
 * Value info:
 *
 *   Error
 *     - Immediate errors represent internal errors, which are:
 *         0: undefined
 *         1: out-of-memory
 *
 *     - Boxed errors represent run-time errors, such as:
 *         * symbol limit
 *         * head of empty list
 *         * tail of empty list
 *         * ...
 *
 *   Symbol
 *     - A symbol contains a pointer to a heap-allocated entry in a hash table,
 *       which stores the symbol data.
 *
 *   List
 *     - A list stores other values in a linked list. An empty list
 *       is an immediate value, while a boxed list contains a value
 *       and the remaining list as a sval_t.
 *
 *   Lambda
 *     - A lambda points to an entry which contains function info,
 *       such as the type of the function (user-defined/builtin),
 *       arity and function body.
 */

#include <stddef.h>
#include <limits.h>

typedef union {
	unsigned long	 u;
	void		*p;
} sval_t;

/* Storage types. */
#define VAL_STORAGE_IMMED	0
#define VAL_STORAGE_BOXED	1

/* Immediate types. */
#define VAL_IMMED_TYPE_ERR	0
#define VAL_IMMED_TYPE_ELIST	1

/* Boxed types. */
#define VAL_BOXED_TYPE_ERR	0
#define VAL_BOXED_TYPE_SYM	1
#define VAL_BOXED_TYPE_LIST	2
#define VAL_BOXED_TYPE_LAMBDA	3

/* Field sizes. */
#define VAL_BITS		(sizeof(sval_t) * 8)
#define VAL_STORAGE_BITS	1
#define VAL_IMMED_TYPE_BITS	1
#define VAL_IMMED_BITS		(VAL_BITS - (VAL_STORAGE_BITS + VAL_IMMED_TYPE_BITS))
#define VAL_BOXED_TYPE_BITS	2
#define VAL_BOXED_BITS		(VAL_BITS - (VAL_STORAGE_BITS + VAL_BOXED_TYPE_BITS))

/* Field offsets. */
#define VAL_STORAGE_OFFSET	0
#define VAL_IMMED_TYPE_OFFSET	(VAL_STORAGE_OFFSET + VAL_STORAGE_BITS)
#define VAL_IMMED_OFFSET	(VAL_IMMED_TYPE_OFFSET + VAL_IMMED_TYPE_BITS)
#define VAL_BOXED_TYPE_OFFSET	(VAL_STORAGE_OFFSET + VAL_STORAGE_BITS)
#define VAL_BOXED_OFFSET	(VAL_BOXED_TYPE_OFFSET + VAL_BOXED_TYPE_BITS)

/* Field masks. */
#define VAL_STORAGE_MASK	(((1 << VAL_STORAGE_BITS) - 1) << VAL_STORAGE_OFFSET)
#define VAL_IMMED_TYPE_MASK	(((1 << VAL_IMMED_TYPE_BITS) - 1) << VAL_IMMED_TYPE_OFFSET)
#define VAL_IMMED_MASK		((unsigned long)~0 << VAL_IMMED_OFFSET)
#define VAL_BOXED_TYPE_MASK	(((1 << VAL_BOXED_TYPE_BITS) - 1) << VAL_BOXED_TYPE_OFFSET)
#define VAL_BOXED_MASK		((unsigned long)~0 << VAL_BOXED_OFFSET)

/* Field limits. */
#define VAL_IMMED_LIM		(ULONG_MAX >> VAL_IMMED_OFFSET)
#define VAL_BOXED_LIM		(ULONG_MAX >> VAL_BOXED_OFFSET)

/* Immediate error types. */
#define VAL_IMMED_ERR_UNDEF	0
#define VAL_IMMED_ERR_NOMEM	1

unsigned long	 get_storage(sval_t);

unsigned long	 get_immed_type(sval_t);
unsigned long	 get_immed(sval_t);
void		 set_immed_empty_list(sval_t *);

unsigned long	 get_boxed_type(sval_t);
void		*get_boxed_ptr(sval_t);

void		*get_boxed_sym_ptr(sval_t);
void		 set_boxed_sym(sval_t *, void *);

void		*get_boxed_list_ptr(sval_t);
void		 set_boxed_list(sval_t *, void *);

void		*get_boxed_lambda_ptr(sval_t);
void		 set_boxed_lambda(sval_t *, void *);

int		 is_immed(sval_t);
int		 is_boxed(sval_t);
int		 is_eq(sval_t, sval_t);

sval_t		 quote(sval_t);
sval_t		 unquote(sval_t);
int		 is_quoted(sval_t);

void		 sval_free(sval_t);

void		 sval_debug(const char *, sval_t);
void		 sval_debug_out(const char *, sval_t);

/*
 * Value-specific function declarations.
 */
#include "err.h"
#include "sym.h"
#include "list.h"
#include "lambda.h"

#endif
