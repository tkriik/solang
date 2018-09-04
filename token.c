#include <assert.h>
#include <ctype.h>
#include <string.h>

#include "conf.h"
#include "token.h"

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

static int
is_sym_char(char c)
{
	return isalnum(c)
	    || (c != '\0' && strchr("<_-/*>!?", c) != NULL);
}

static int
is_sym_start(char c)
{
	return isalpha(c)
	    || (c != '\0' && strchr("<_-/*>!?", c) != NULL);
}

enum list_char {
	LIST_CHAR_START	= '(',
	LIST_CHAR_END	= ')'
};

static int
is_list_char(char c)
{
	return c == LIST_CHAR_START || c == LIST_CHAR_END;
}

enum token_type
list_char_token_type(char c)
{
	switch (c) {
	case LIST_CHAR_START:	return TOKEN_TYPE_LIST_START;
	case LIST_CHAR_END:	return TOKEN_TYPE_LIST_END;
	}

	assert(0 && "NOTREACHED");
	return -1;
}

enum token_res
token_next(const char **srcp, struct token_info *token)
{
	assert(srcp != NULL);
	assert(*srcp != NULL);
	assert(token != NULL);

	/*
	 * State transitions, from current state to most likely next state:
	 *
	 *   NEXT_TOKEN -> NEXT_TOKEN
	 *               | AT_LIST
	 *               | AT_SYM
	 *               | <return>
	 *               | AT_ERR
	 *
	 *   AT_LIST    -> <return>
	 *
	 *   AT_SYM     -> AT_SYM
	 *               | <return>
	 *               | AT_ERR
	 *
	 *   AT_ERR     -> AT_ERR
	 *               | <return>
	 */
	enum {
		NEXT_TOKEN,
		AT_LIST,
		AT_SYM,
		AT_ERR
	} state = NEXT_TOKEN;

	const char *cur = *srcp;
	while (1) {
		char c = *cur;

		switch (state) {
		case NEXT_TOKEN:
			/* NEXT_TOKEN -> NEXT_TOKEN */
			if (is_whitespace(c)) {
				cur++;
				continue;
			}

			/* NEXT_TOKEN -> AT_LIST */
			if (is_list_char(c)) {
				state = AT_LIST;
				continue;
			}

			/* NEXT_TOKEN -> AT_SYM */
			if (is_sym_start(c)) {
				token->type = TOKEN_TYPE_SYM;
				token->len = 1;
				token->src = cur;
				state = AT_SYM;
				cur++;
				continue;
			}

			/* NEXT_TOKEN -> <return> */
			if (is_end(c)) {
				*srcp = cur;
				return TOKEN_RES_NONE;
			}

			/* NEXT_TOKEN -> AT_ERR */
			token->type = TOKEN_TYPE_ERR;
			token->len = 1;
			token->src = cur;
			state = AT_ERR;
			cur++;
			continue;

		case AT_LIST:
			/* AT_LIST -> <return> */
			token->type = list_char_token_type(c);
			token->len = 1;
			token->src = cur;
			*srcp = cur + 1;
			return TOKEN_RES_OK;

		case AT_SYM:
			/*
			 * AT_SYM -> AT_SYM
			 */
			if (is_sym_char(c)) {
				token->len++;
				if (SYM_MAX_LEN < token->len)
					token->type = TOKEN_TYPE_ERR;
				cur++;
				continue;
			}

			/* AT_SYM -> <return> */
			if (is_list_char(c) || is_whitespace(c) || is_end(c)) {
				*srcp = cur;
				return TOKEN_RES_OK;
			}

			/* AT_SYM -> AT_ERR */
			token->type = TOKEN_TYPE_ERR;
			token->len++;
			state = AT_ERR;
			cur++;
			continue;

		case AT_ERR:
			/* AT_ERR -> <return> */
			if (is_list_char(c) || is_whitespace(c) || is_end(c)) {
				*srcp = cur;
				return TOKEN_RES_OK;
			}

			/* AT_ERR -> AT_ERR */
			token->len++;
			cur++;
			continue;

		};
	}

	assert(0 && "NOTREACHED");
	return TOKEN_RES_NONE;
}
