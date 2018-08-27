#ifndef PARSE_H
#define PARSE_H

#include "token.h"
#include "val.h"

enum parse_res {
	PARSE_RES_OK	= 1,
	PARSE_RES_ERR
};

enum parse_res	 parse_token(struct token_info *, val_t *);

const char	*parse_res_str(enum parse_res);

#endif
