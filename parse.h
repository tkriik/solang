#ifndef PARSE_H
#define PARSE_H

#include "token.h"
#include "val.h"

enum parse_result {
	/* Parsed one value, no more tokens */
	PARSE_OK = 1,

	/* Parsed one value, tokens remaining */
	PARSE_CONT
};

enum parse_result parse(struct token_info *, size_t, val_t *, struct token_info **);

#endif
