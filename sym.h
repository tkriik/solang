#ifndef SYM_H
#define SYM_H

#include <stddef.h>

#include "sds.h"

typedef char *sym_t;

sym_t		sym_alloc(const char *);
void		sym_free(sym_t);

const char	*sym_str(sym_t);

#endif
