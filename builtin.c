#include "builtin.h"
#include "val.h"

void
builtin_init(void)
{
	BUILTIN.quote.sym = sym("quote");
	BUILTIN.quote.fn = NULL;
}
