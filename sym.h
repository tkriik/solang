#ifndef SOLANG_SYM_H
#define SOLANG_SYM_H

#include "sval.h"

sval_t		 sym(const char *);
sval_t		 symn(const char *, size_t);

int		 is_sym(sval_t);

const char	*sym_name(sval_t);

#endif
