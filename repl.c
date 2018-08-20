#include <err.h>
#include <stdlib.h>
#include <readline/history.h>
#include <readline/readline.h>

#include "repl.h"
#include "tal.h"

static void loop(void);
static void handle_input(const char *);
static void handle_command(const char *);
static void print_help(void);

void
repl_enter(void)
{
	printf("tal (Tanel's Language) version %d.%d.%d\n",
	    VSN_MAJOR, VSN_MINOR, VSN_PATCH);

	print_help();

	loop();
}

static void
loop(void)
{
	while (1) {
		char *input = readline(">> ");
		if (input == NULL)
			err(0, "EOF while reading standard input");

		add_history(input);

		handle_input(input);

		free(input);
	}
}

static void
handle_input(const char *input)
{
	if (strncmp(input, "\\", 1) == 0)
		handle_command(input);
}

static void
handle_command(const char *command)
{
	if (strcmp(command, "\\h") == 0)
		print_help();

	if (strcmp(command, "\\q") == 0)
		exit(0);
}

static void
print_help(void)
{
	printf("\n"
	       "\\h - help\n"
	       "\\q - quit\n"
	       "\n");
}
