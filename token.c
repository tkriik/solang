#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "token.h"

static int
is_sym_char(char c)
{
	return isalnum(c) || strchr("<_-/*>!?", c) != NULL;
}

static int
is_sym_start(char c)
{
	return isalpha(c) || strchr("<_-/*>!?", c) != NULL;
}

static int
is_whitespace(char c)
{
	return isspace(c);
}

static int
is_end(char c)
{
	return c == '\0';
}

enum tokenize_result
tokenize(const char *src, struct token_info *token_buf,
    size_t max_tokens, size_t *ntokens)
{
	assert(src != NULL);
	assert(token_buf != NULL);
	assert(0 < max_tokens);
	assert(ntokens != NULL);

	const char *s = src;
	size_t token_idx = 0;

	*ntokens = 0;

	enum {
		NEXT_TOKEN,
		AT_NULL,
		AT_SYM,
		AT_ERR
	} state = NEXT_TOKEN;

	do {
		if (token_idx >= max_tokens)
			return TOKENIZE_LIMIT;

		if (is_end(*s))
			break;

		struct token_info *token = &token_buf[token_idx];

		/*
		 * State transitions:
		 *
		 *   NEXT_TOKEN -> NEXT_TOKEN | AT_SYM | AT_ERR
		 *   AT_SYM     -> AT_SYM | AT_NULL | NEXT_TOKEN | AT_ERR
		 *   AT_NULL    -> NEXT_TOKEN | AT_SYM | AT_ERR
		 *   AT_ERR     -> AT_ERR | NEXT_TOKEN
		 */
		switch (state) {
		case NEXT_TOKEN:
			if (is_whitespace(*s))
				continue;

			if (is_sym_start(*s)) {
				state = AT_SYM;
				token->type = TOKEN_SYM;
				token->len = 1;
				token->src = s;
				(*ntokens)++;
				continue;
			}

			state = AT_ERR;
			token->type = TOKEN_ERR;
			token->len = 1;
			token->src = s;
			(*ntokens)++;
			continue;

		case AT_NULL:
			if (is_whitespace(*s)) {
				state = NEXT_TOKEN;
				token_idx++;
				continue;
			}

			if (is_sym_char(*s)) {
				token->len++;
				token->type = AT_SYM;
				continue;
			}

			state = AT_ERR;
			token->type = TOKEN_ERR;
			token->len++;
			continue;

		case AT_SYM:
			if (is_sym_char(*s)) {
				token->len++;
				if (token->len == 4 &&
				    strncmp(token->src, "null", 4) == 0) {
					token->type = TOKEN_NULL;
					state = AT_NULL;
				}
				continue;
			}

			if (is_whitespace(*s)) {
				state = NEXT_TOKEN;
				token_idx++;
				continue;
			}

			token->type = TOKEN_ERR;
			token->len++;
			continue;

		case AT_ERR:
			if (is_whitespace(*s)) {
				state = NEXT_TOKEN;
				token_idx++;
				continue;
			}

			token->len++;
			continue;
		}
	} while (*s++ != '\0');

	return TOKENIZE_OK;
}

const char *
token_type_str(enum token_type type)
{
	switch (type) {
	case TOKEN_NULL:	return "NULL";
	case TOKEN_SYM:		return "SYM";
	case TOKEN_ERR:		return "ERR";
	default:		return "UNKNOWN";
	}
}

void
token_debug(const char *info, struct token_info *tokens, size_t ntokens)
{
	assert(tokens != NULL);

	printf("-------- %s\n", info);

	printf("{\n");
	for (size_t i = 0; i < ntokens; i++) {
		struct token_info *token = &tokens[i];

		if (i == 0)
			printf("\t{\n");

		char *src_data = strndup(token->src, token->len);
		printf(
		    "\t\t.type = %s\n"
		    "\t\t.len  = %zu\n"
		    "\t\t.src  = %p \"%s\"\n",
		    token_type_str(token->type),
		    token->len,
		    token->src, src_data);
		free(src_data);

		if (i < ntokens - 1)
			printf("\t}, {\n");
		else
			printf("\t}\n");
	}

	printf("}\n");

	printf("--------\n");
}
