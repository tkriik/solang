#include "builtin.h"
#include "val.h"

void
builtin_init(void)
{
	BUILTIN.sym.quote = sym("quote");
}
