#ifndef SOLANG_LIST_H
#define SOLANG_LIST_H

sval_t	list(void);
sval_t	nonempty_list(sval_t, sval_t);

int	is_empty_list(sval_t);
int	is_nonempty_list(sval_t);
int	is_list(sval_t);

int	is_pair(sval_t);
int	is_triple(sval_t);

sval_t	cons(sval_t, sval_t);
sval_t	car(sval_t);
sval_t	cdr(sval_t);

sval_t  snoc_tail(sval_t, sval_t);

size_t	list_count(sval_t);
sval_t	list_reverse_inplace(sval_t);

int	nonempty_list_eq(sval_t, sval_t);
void	nonempty_list_free(sval_t);

/*
 * Cannot properly assign both v and l in the same update statement,
 * so do away with this hack.
 */
#define LIST_FOREACH(v, l)						\
	for (int _once = 1; !is_empty_list(l); (l) = cdr(l), _once = 1)	\
		for ((v) = car(l); _once; _once = 0)

#endif
