#include <string.h>

#define MUNIT_ENABLE_ASSERT_ALIASES
#include "munit.h"

#include "sym.h"

static void
test_str(const char *expected)
{
	sym_t sym = sym_alloc(expected);
	const char *actual = sym_str(sym);

	assert_string_equal(expected, actual);

	sym_free(sym);
}

static void
test_empty(void)
{
	test_str("");
}

static void
test_short(void)
{
	test_str("short_symbol");
}

static void
test_long(void)
{
	char long_str[4096 + 1];
	memset(long_str, 'A', sizeof(long_str) - 1);
	long_str[4096] = '\0';

	test_str(long_str);
}

void
test_sym(void)
{
	test_empty();
	test_short();
	test_long();
}
