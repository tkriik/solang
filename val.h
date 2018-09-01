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
 *       and the remaining list as a val_t.
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
} val_t;

enum val_storage {
	VAL_STORAGE_IMMED	= 0,
	VAL_STORAGE_BOXED	= 1
};

enum val_immed_type {
	VAL_IMMED_TYPE_ERR	= 0,
	VAL_IMMED_TYPE_ELIST	= 1
};

enum val_boxed_type {
	VAL_BOXED_TYPE_ERR	= 0,
	VAL_BOXED_TYPE_SYM	= 1,
	VAL_BOXED_TYPE_LIST	= 2,
	VAL_BOXED_TYPE_LAMBDA	= 3
};

enum val_bits {
	VAL_BITS		= sizeof(val_t) * 8,
	VAL_STORAGE_BITS	= 1,
	VAL_IMMED_TYPE_BITS	= 1,
	VAL_IMMED_BITS		= VAL_BITS - (VAL_STORAGE_BITS + VAL_IMMED_TYPE_BITS),
	VAL_BOXED_TYPE_BITS	= 2,
	VAL_BOXED_BITS		= VAL_BITS - (VAL_STORAGE_BITS + VAL_BOXED_TYPE_BITS)
};

enum val_offset {
	VAL_STORAGE_OFFSET	= 0,
	VAL_IMMED_TYPE_OFFSET	= VAL_STORAGE_OFFSET + VAL_STORAGE_BITS,
	VAL_IMMED_OFFSET	= VAL_IMMED_TYPE_OFFSET + VAL_IMMED_TYPE_BITS,
	VAL_BOXED_TYPE_OFFSET	= VAL_STORAGE_OFFSET + VAL_STORAGE_BITS,
	VAL_BOXED_OFFSET	= VAL_BOXED_TYPE_OFFSET + VAL_BOXED_TYPE_BITS
};

enum val_mask {
	VAL_STORAGE_MASK	= ((1 << VAL_STORAGE_BITS) - 1) << VAL_STORAGE_OFFSET,
	VAL_IMMED_TYPE_MASK	= ((1 << VAL_IMMED_TYPE_BITS) - 1) << VAL_IMMED_TYPE_OFFSET,
	VAL_IMMED_MASK		= (unsigned long)~0 << VAL_IMMED_OFFSET,
	VAL_BOXED_TYPE_MASK	= ((1 << VAL_BOXED_TYPE_BITS) - 1) << VAL_BOXED_TYPE_OFFSET,
	VAL_BOXED_MASK		= (unsigned long)~0 << VAL_BOXED_OFFSET
};

enum val_lim {
	VAL_IMMED_LIM		= ULONG_MAX >> VAL_IMMED_OFFSET,
	VAL_BOXED_LIM		= ULONG_MAX >> VAL_BOXED_OFFSET
};

enum val_immed_err_type {
	VAL_IMMED_ERR_UNDEF	= 0,
	VAL_IMMED_ERR_NOMEM	= 1
};

/*
 * val_util.c
 */
unsigned long	 get_storage(val_t);

unsigned long	 get_immed_type(val_t);
unsigned long	 get_immed(val_t);
void		 set_immedempty_list(val_t *);

unsigned long	 get_boxed_type(val_t);
void		*get_boxed_ptr(val_t);

void		*get_boxed_sym_ptr(val_t);
void		 set_boxed_sym(val_t *, void *);

void		*get_boxed_list_ptr(val_t);
void		 set_boxed_list(val_t *, void *);

void		*get_boxed_lambda_ptr(val_t);
void		 set_boxed_lambda(val_t *, void *);

/*
 * val.c
 */
int		 is_immed(val_t);
int		 is_boxed(val_t);
int		 is_eq(val_t, val_t);

val_t		 quote(val_t);
val_t		 unquote(val_t);
int		 is_quoted(val_t);

void		 val_free(val_t);

/*
 * err.c
 */
val_t		 err_undef(void);
int		 is_err(val_t);
int		 is_err_undef(val_t);
const char	*err_str(val_t);

/*
 * sym.c
 */
val_t		 sym(const char *);
val_t		 symn(const char *, size_t);
const char	*sym_name(val_t);

int		 is_sym(val_t);

/*
 * list.c
 */
val_t		 empty_list(void);
val_t		 nonempty_list(val_t, val_t);
val_t		 list(void);

int		 is_empty_list(val_t);
int		 is_nonempty_list(val_t);
int		 is_list(val_t);

int		 is_pair(val_t);
int		 is_triple(val_t);

val_t		 cons(val_t, val_t);
val_t		 car(val_t);
val_t		 cdr(val_t);

size_t		 list_count(val_t);
val_t		 list_reverse_inplace(val_t);

int		 nonempty_list_eq(val_t, val_t);
void		 nonempty_list_free(val_t);

/*
 * Cannot properly assign both v and l in the same update statement,
 * so do away with this hack.
 */
#define LIST_FOREACH(v, l)						\
	for (int _once = 1; !is_empty_list(l); (l) = cdr(l), _once = 1)	\
		for ((v) = car(l); _once; _once = 0)

/*
 * lambda.c
 */
struct env;

typedef val_t (*builtin_fn)(struct env *env, val_t);

val_t		  lambda_builtin(builtin_fn, size_t);
val_t		  lambda_apply(struct env *, val_t, val_t);

int		  is_lambda_builtin(val_t);

const char	 *lambda_type_str(val_t);
void		  lambda_free(val_t);
void		  lambda_free_builtin(val_t);

/*
 * val_debug.c
 */
void		  val_debug(const char *, val_t);

#endif
