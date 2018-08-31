#include <assert.h>

#include "parse.h"
#include "token.h"
#include "val.h"

struct state {
	const char	*src;
	long		 level;
};

/* TODO: free on error */
static val_t
do_parse(struct state *st)
{
	assert(st != NULL);
	assert(st->src != NULL);

	val_t l = list();

	long cur_level = st->level;

	struct token_info token;
	for (enum token_res tres = token_next(&st->src, &token);
	     tres != TOKEN_RES_NONE;
	     tres = token_next(&st->src, &token)) {

		val_t v = err_undef();

		switch (token.type) {
		case TOKEN_TYPE_SYM:
			v = symn(token.src, token.len);
			break;

		case TOKEN_TYPE_LIST_START:
			st->level++;
			v = do_parse(st);
			if (is_err_undef(v)) {
				val_free(l);
				return v;
			}
			break;

		case TOKEN_TYPE_LIST_END:
			if (0 < st->level) {
				st->level--;
				l = list_reverse_inplace(l);
				return l;
			}

			val_free(l);
			return err_undef();

		case TOKEN_TYPE_ERR:
			val_free(l);
			return err_undef();

		default:
			assert(0 && "NOTREACHED");
		}

		l = cons(v, l);
	}

	if (cur_level != st->level) {
		val_free(l);
		return err_undef();
	}

	l = list_reverse_inplace(l);

	return l;
}

val_t
parse(const char *src)
{
	assert(src != NULL);

	struct state st = {
		.src	= src,
		.level	= 0
	};

	val_t l = do_parse(&st);
	assert(is_list(l) || is_err_undef(l));

	return l;
}
