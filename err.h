#ifndef SOLANG_ERR_H
#define SOLANG_ERR_H

#include "sval.h"

sval_t		 err_undef(void);

int		 is_err(sval_t);
int		 is_err_undef(sval_t);

const char	*err_str(sval_t);

#endif
