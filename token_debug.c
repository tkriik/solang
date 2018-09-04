#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "token.h"

void
token_debug(const char *info, struct token_info *token)
{
	assert(info != NULL);
	assert(token != NULL);

	printf("-------- %s\n", info);
	printf("{\n");

	char *src_data = calloc(1, token->len + 1);
	assert(src_data != NULL);
	memcpy(src_data, token->src, token->len);

	printf(
	    "\t\t.type = %s\n"
	    "\t\t.len  = %zu\n"
	    "\t\t.src  = %p \"%s\"\n",
	    token_type_str(token->type),
	    token->len,
	    (void *)token->src, src_data);

	free(src_data);

	printf("}\n");
	printf("--------\n");
}
