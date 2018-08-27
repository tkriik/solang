#include <err.h>
#include <stdlib.h>
#include <string.h>
#include <readline/history.h>
#include <readline/readline.h>

#include "sds.h"

#include "parse.h"
#include "repl.h"
#include "tal.h"
#include "token.h"
#include "val.h"

enum cmd_type {
	CMD_CONFIG,
	CMD_DEBUG_TOKENS,
	CMD_DEBUG_VAL,
	CMD_HELP,
	CMD_QUIT
};

struct cmd_info {
	enum		  cmd_type type;
	const char	 *name;
	int		  arity;
	void		(*handler)(sds *);
};

static void config_handler();
static void debug_value_handler(sds *);
static void debug_token_handler(sds *);
static void help_handler();
static void quit_handler();

#define CMD_CNT 5
static struct cmd_info CMD_INFO_TAB[CMD_CNT] = {
	{
		.type		= CMD_CONFIG,
		.name		= "\\c",
		.arity		= 0,
		.handler	= config_handler
	}, {
		.type		= CMD_DEBUG_VAL,
		.name		= "\\dv",
		.arity		= 1,
		.handler	= debug_value_handler
	}, {
		.type		= CMD_DEBUG_TOKENS,
		.name		= "\\dt",
		.arity		= 1,
		.handler	= debug_token_handler
	}, {
		.type		= CMD_HELP,
		.name		= "\\h",
		.arity		= 0,
		.handler	= help_handler
	}, {
		.type		= CMD_QUIT,
		.name		= "\\q",
		.arity		= 0,
		.handler	= quit_handler
	}
};

static const char *CMD_HELP_MSG =
    "\n"
    "\\c           - print REPL configuration\n"
    "\\dt [on|off] - turn token debugging on/off\n"
    "\\dv [on|off] - turn value debugging on/off\n"
    "\\h           - help\n"
    "\\q           - quit\n"
    "\n";

static struct {
	int debug_value;
	int debug_token;
} config = {
	.debug_value	= 0,
	.debug_token	= 0
};

static void
config_handler()
{
	printf(
	    "\n"
	    "debug_value = %d\n"
	    "debug_token = %d\n"
	    "\n",
	    config.debug_value,
	    config.debug_token);
}

static void
debug_value_handler(sds *argv)
{
	sds mode = argv[0];
	if (strcmp(mode, "on") == 0)
		config.debug_value = 1;
	else if (strcmp(mode, "off") == 0)
		config.debug_value = 0;
	else {
		printf("no such value debug mode: %s\n", mode);
		return;
	}
}

static void
debug_token_handler(sds *argv)
{
	sds mode = argv[0];
	if (strcmp(mode, "on") == 0)
		config.debug_token = 1;
	else if (strcmp(mode, "off") == 0)
		config.debug_token = 0;
	else {
		printf("no such token debug mode: %s\n", mode);
		return;
	}
}

static void
help_handler()
{
	printf("%s", CMD_HELP_MSG);
}

static void
quit_handler()
{
	exit(0);
}

static void
handle_command(sds input)
{
	int ntokens;
	sds *tokens = sdssplitargs(input, &ntokens);
	sds cmd = tokens[0];
	int argc = ntokens - 1;

	int handled = 0;
	for (size_t i = 0; i < CMD_CNT; i++) {
		struct cmd_info *cmd_info = &CMD_INFO_TAB[i];

		if (strcmp(cmd, cmd_info->name) == 0) {
			if (cmd_info->arity == argc) {
				cmd_info->handler(&tokens[1]);
			} else {
				printf("command %s expects %d argument(s)\n",
				    cmd_info->name, cmd_info->arity);
			}

			handled = 1;
			break;
		}
	}

	if (handled == 0)
		printf("unknown command: %s\n", tokens[0]);
}

static void
eval(sds input)
{
	const char *src = input;
	struct token_info token;
	enum token_res tres;

	int multi_eval = 0;
	do {
		tres = token_next((const char **)&src, &token);
		if (tres == TOKEN_RES_NONE) {
			if (!multi_eval && config.debug_token)
				printf("no tokens read\n");
			break;
		}

		if (config.debug_token)
			token_debug("token", &token);

		val_t v;
		enum parse_res pres = parse_token(&token, &v);
		if (pres == PARSE_RES_ERR) {
			printf("parse error\n");
			continue;
		}

		if (config.debug_value)
			val_debug("value", v);

		val_free(v);

		multi_eval = 1;
	} while (tres == TOKEN_RES_OK);
}

static void
handle_input(sds input)
{
	if (strncmp(input, "\\", 1) == 0)
		handle_command(input);
	else
		eval(input);
}


static void
loop(void)
{
	while (1) {
		sds input = sdsnew(readline(">> "));
		if (input == NULL)
			err(0, "EOF while reading standard input");

		input = sdstrim(input, " \t");

		add_history(input);
		handle_input(input);

		sdsfree(input);
	}
}

void
repl_enter(void)
{
	printf("tal (Tanel's Language) version %d.%d.%d\n",
	    VSN_MAJOR, VSN_MINOR, VSN_PATCH);

	help_handler();

	loop();
}
