#ifndef SOLANG_SYM_H
#define SOLANG_SYM_H

#include "sval.h"

/* Maximun number of symbols */
#define SYM_MAX_CNT	(1 << 20)

/* Symbol length limit (not including null terminator) */
#define SYM_MAX_LEN	255

sval_t		 sym(const char *);
sval_t		 symn(const char *, size_t);

int		 is_sym(sval_t);

const char	*sym_name(sval_t);

#endif
