#include <assert.h>
#include <limits.h>

#include "parse.h"
#include "token.h"
#include "sval.h"

struct state {
	const char	*src;
	long		 level;
};

/* TODO: free on error */
static sval_t
do_parse(struct state *st)
{
	assert(st != NULL);
	assert(st->src != NULL);

	sval_t l = list();
	sval_t l_tail = l;

	long cur_level = st->level;
	unsigned long quotes = 0;

	struct token_info token;
	for (enum token_res tres = token_next(&st->src, &token);
	     tres != TOKEN_RES_NONE;
	     tres = token_next(&st->src, &token)) {

		sval_t v = err_undef();

		switch (token.type) {
		case TOKEN_TYPE_SYM:
			v = symn(token.src, token.len);
			break;

		case TOKEN_TYPE_LIST_START:
			st->level++;
			v = do_parse(st);
			if (is_err_undef(v)) {
				sval_free(l);
				return v;
			}
			break;

		case TOKEN_TYPE_LIST_END:
			if (0 < st->level) {
				st->level--;
				return l;
			}

			sval_free(l);
			return err_undef();

		case TOKEN_TYPE_QUOTE:
			assert(quotes < ULONG_MAX);
			quotes++;
			continue;

		case TOKEN_TYPE_ERR:
			sval_free(l);
			return err_undef();

		default:
			assert(0 && "NOTREACHED");
		}

		for (size_t i = quotes; 0 < i; i--) {
			v = quote(v);
			quotes--;
		}

		l_tail = snoc_tail(l_tail, v);
		if (is_empty_list(l))
			l = l_tail;
	}

	if (cur_level != st->level) {
		sval_free(l);
		return err_undef();
	}

	return l;
}

sval_t
parse(const char *src)
{
	assert(src != NULL);

	struct state st = {
		.src	= src,
		.level	= 0
	};

	sval_t l = do_parse(&st);
	assert(is_list(l) || is_err_undef(l));

	return l;
}
