#include <assert.h>
#include <stdlib.h>

#include "val.h"

enum lambda_type {
	LAMBDA_TYPE_BUILTIN = 1
};

struct lambda {
	int	arity;
	enum	lambda_type type;
	union {
		builtin_fn builtin_fn;
	} u;
};

val_t
lambda_builtin(unsigned long arity, builtin_fn fn)
{
	assert(fn != NULL);

	struct lambda *lambda = malloc(sizeof(*lambda));
	assert(lambda != NULL);

	lambda->arity = arity;
	lambda->type = LAMBDA_TYPE_BUILTIN;
	lambda->u.builtin_fn = fn;

	val_t v = err_undef();
	_set_boxed_lambda(&v, lambda);

	return v;
}

int
is_lambda(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_BOXED
	    && _get_boxed_type(v) == VAL_BOXED_TYPE_LAMBDA;
}

int
is_lambda_builtin(val_t v)
{
	if (!is_lambda(v))
		return 0;

	struct lambda *lambda = _get_boxed_lambda_ptr(v);

	return lambda->type == LAMBDA_TYPE_BUILTIN;
}

void
lambda_free(val_t v)
{
	assert(is_lambda(v));

	struct lambda *lambda = _get_boxed_lambda_ptr(v);
	free(lambda);
}

const char *
lambda_type_str(val_t v)
{
	assert(is_lambda(v));

	struct lambda *lambda = _get_boxed_lambda_ptr(v);
	switch (lambda->type) {
	case LAMBDA_TYPE_BUILTIN:	return "LAMBDA_TYPE_BUILTIN";
	default:			return "LAMBDA_TYPE_<INVALID>";
	}
}
