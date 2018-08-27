#ifndef TOKEN_H
#define TOKEN_H

#include <stddef.h>

enum token_type {
	TOKEN_TYPE_NULL	= 1,
	TOKEN_TYPE_SYM,
	TOKEN_TYPE_ERR
};

struct token_info {
	enum token_type	 type;
	size_t		 len;
	const char	*src;
};

enum token_res {
	TOKEN_RES_OK	= 1,
	TOKEN_RES_NONE
};

/* token.c */
enum token_res	 token_next(const char **, struct token_info *);

/* token_debug.c */
const char	*token_type_str(enum token_type);
void		 token_debug(const char *, struct token_info *);

#endif
