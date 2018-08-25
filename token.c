#include <assert.h>
#include <ctype.h>
#include <stdio.h>
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
		SYM,
		ERR
	} state = NEXT_TOKEN;

	do {
		if (token_idx >= max_tokens)
			return TOKENIZE_LIMIT;

		if (is_end(*s))
			break;

		struct token_info *token = &token_buf[token_idx];

		switch (state) {
		case NEXT_TOKEN:
			if (is_whitespace(*s))
				continue;

			if (is_sym_start(*s)) {
				state = SYM;
				token->type = TOKEN_SYM;
				token->len = 1;
				token->src = s;
				(*ntokens)++;
				continue;
			}

			state = ERR;
			token->type = TOKEN_ERR;
			token->len = 1;
			token->src = s;
			(*ntokens)++;
			continue;

		case SYM:
			if (is_sym_char(*s)) {
				token->len++;
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

		case ERR:
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
	case TOKEN_SYM:	return "SYM";
	case TOKEN_ERR:	return "ERR";
	default:	return "UNKNOWN";
	}
}

void
token_debug(struct token_info *token)
{
	assert(token != NULL);

	char src_data[token->len + 1];
	memcpy(src_data, token->src, token->len);
	src_data[token->len] = '\0';

	printf(
	    "\n"
	    "token_info {\n"
	    "        .type = %s\n"
	    "        .len  = %zu\n"
	    "        .src  = %p \"%s\"\n"
	    "}\n",
	    token_type_str(token->type),
	    token->len,
	    token->src, src_data);
}
