#ifndef TOKEN_H
#define TOKEN_H

#include <stddef.h>

enum token_type {
	TOKEN_SYM = 1,
	TOKEN_ERR
};

struct token_info {
	enum		 token_type type;
	size_t		 len;
	const char	*src;
};

enum tokenize_result {
	TOKENIZE_OK	= 1,
	TOKENIZE_LIMIT
};

enum tokenize_result tokenize(const char *, struct token_info *, size_t, size_t *);

void token_debug(struct token_info *, size_t);

#endif
