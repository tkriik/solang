#include "env.h"
#include "sval.h"

sval_t eval(struct env *, sval_t);
sval_t eval_src(struct env *, const char *);
