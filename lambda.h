#ifndef SOLANG_LAMBDA_H
#define SOLANG_LAMBDA_H

#include "sval.h"

struct env;

typedef sval_t (*builtin_fn)(struct env *env, sval_t);

sval_t		 lambda_builtin(builtin_fn, size_t);
sval_t		 lambda_apply(struct env *, sval_t, sval_t);

int		 is_lambda_builtin(sval_t);

const char	*lambda_type_str(sval_t);
void		 lambda_free(sval_t);
void		 lambda_free_builtin(sval_t);

#endif
