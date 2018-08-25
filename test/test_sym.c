#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

void
test_sym(void)
{
	assert_int(1, ==, 2);
}
