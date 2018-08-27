#include <assert.h>

#include "parse.h"
#include "token.h"
#include "val.h"

enum parse_res
parse_token(struct token_info *token, val_t *vp)
{
	assert(token != NULL);
	assert(vp != NULL);

	switch (token->type) {
	case TOKEN_TYPE_NULL:
		*vp = mk_null();
		return PARSE_RES_OK;

	case TOKEN_TYPE_SYM:
		*vp = mk_sym(token->src, token->len);
		return PARSE_RES_OK;

	case TOKEN_TYPE_ERR:
		*vp = _mk_undef();
		return PARSE_RES_ERR;
	}

	assert(0 && "NOTREACHED");
	return PARSE_RES_ERR;
}

const char *
parse_res_str(enum parse_res res)
{
	switch (res) {
	case PARSE_RES_OK:	return "PARSE_RES_OK";
	case PARSE_RES_ERR:	return "PARSE_RES_ERR";
	default:		return "PARSE_RES_<INVALID>";
	}
}
