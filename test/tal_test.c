#include <stddef.h>

#include "munit.h"

extern MunitTest sym_tests[];

static MunitSuite suites[] = {
	{
		.prefix		= "/sym",
		.tests		= sym_tests,
		.suites		= NULL,
		.iterations	= 1,
		.options	= MUNIT_SUITE_OPTION_NONE
	},
	{ NULL, NULL, NULL, 0, MUNIT_SUITE_OPTION_NONE }
};

static const MunitSuite suite = {
	.prefix		= "/tal",
	.tests		= NULL,
	.suites		= suites,
	.iterations	= 1,
	.options	= MUNIT_SUITE_OPTION_NONE
};

int
main(int argc, char *argv[])
{
	return munit_suite_main(&suite, NULL, argc, argv);
}
