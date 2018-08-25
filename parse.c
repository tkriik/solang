#include <assert.h>

#include "parse.h"
#include "token.h"
#include "val.h"

enum parse_result
parse(struct token_info *tokens, size_t ntokens, val_t *vp,
    struct token_info **next_tokensp)
{
	assert(tokens != NULL);
	assert(0 < ntokens);
	assert(vp != NULL);
	assert(next_tokensp != NULL);

	enum parse_result result;

	size_t token_idx;
	for (token_idx = 0; token_idx < ntokens; token_idx++) {
		struct token_info *token = &tokens[token_idx];

		switch (token->type) {
		case TOKEN_NULL:
			*vp = mk_null();
			result = PARSE_OK;
			goto finish;

		case TOKEN_SYM:
			*vp = mk_sym(token->src, token->len);
			result = PARSE_OK;
			goto finish;

		case TOKEN_ERR:
			/* We assume erroneous tokens are handled before this point */
			assert(0 && "NOTREACHED");
			goto finish;
		}
	}

finish:
	if (token_idx < ntokens - 1) {
		*next_tokensp = &tokens[token_idx + 1];
		result = PARSE_CONT;
	} else {
		*next_tokensp = NULL;
		result = PARSE_OK;
	}

	return result;
}
