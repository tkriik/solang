#include <assert.h>
#include <stdlib.h>

#include "env.h"
#include "sval.h"

enum lambda_type {
	LAMBDA_TYPE_BUILTIN = 1
};

struct lambda {
	size_t	arity;
	enum	lambda_type type;
	union {
		builtin_fn builtin_fn;
	} u;
};

sval_t
lambda_builtin(builtin_fn fn, size_t arity)
{
	assert(fn != NULL);

	struct lambda *lambda = malloc(sizeof(*lambda));
	assert(lambda != NULL);

	lambda->arity = arity;
	lambda->type = LAMBDA_TYPE_BUILTIN;
	lambda->u.builtin_fn = fn;

	sval_t v = err_undef();
	set_boxed_lambda(&v, lambda);

	return v;
}

int
is_lambda(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED
	    && get_boxed_type(v) == VAL_BOXED_TYPE_LAMBDA;
}

size_t
lambda_arity(sval_t v)
{
	struct lambda *lambda = get_boxed_lambda_ptr(v);
	return lambda->arity;
}

int
is_lambda_builtin(sval_t v)
{
	if (!is_lambda(v))
		return 0;

	struct lambda *lambda = get_boxed_lambda_ptr(v);

	return lambda->type == LAMBDA_TYPE_BUILTIN;
}

sval_t
lambda_apply(struct env *env, sval_t v, sval_t args)
{
	assert(is_lambda(v));
	assert(is_list(args));

	struct lambda *lambda = get_boxed_lambda_ptr(v);

	size_t argc = list_count(args);
	if (argc != lambda->arity)
		return err_undef(); /* TODO: err_arity */

	switch (lambda->type) {
	case LAMBDA_TYPE_BUILTIN:
		return lambda->u.builtin_fn(env, args);
	}

	assert(0 && "NOTREACHED");
}

void
lambda_free(sval_t v)
{
	assert(is_lambda(v));

	struct lambda *lambda = get_boxed_lambda_ptr(v);
	switch (lambda->type) {
	case LAMBDA_TYPE_BUILTIN:
		return;
	/* TODO: free user-lambda */
	}
}

void
lambda_free_builtin(sval_t v)
{
	assert(is_lambda_builtin(v));

	struct lambda *lambda = get_boxed_lambda_ptr(v);
	free(lambda);
}

const char *
lambda_type_str(sval_t v)
{
	assert(is_lambda(v));

	struct lambda *lambda = get_boxed_lambda_ptr(v);
	switch (lambda->type) {
	case LAMBDA_TYPE_BUILTIN:	return "LAMBDA_TYPE_BUILTIN";
	default:			return "LAMBDA_TYPE_<INVALID>";
	}
}
