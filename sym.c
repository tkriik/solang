#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "uthash.h"

#include "conf.h"
#include "sval.h"

struct sym_entry {
	char		name[SYM_MAX_LEN + 1];
	UT_hash_handle	hh;
};

static struct sym_entry *sym_entries = NULL;

sval_t
sym(const char *name)
{
	return symn(name, strlen(name));
}

sval_t
symn(const char *name, size_t len)
{
	assert(0 < len);
	assert(len <= SYM_MAX_LEN);
	assert(HASH_COUNT(sym_entries) <= SYM_MAX_CNT);

	sval_t sym = err_undef();

	struct sym_entry *entry;
	HASH_FIND(hh, sym_entries, name, len, entry);
	if (entry != NULL) {
		set_boxed_sym(&sym, entry);
		return sym;
	}

	entry = malloc(sizeof(*entry));
	assert(entry != NULL);
	memset(entry->name, '\0', sizeof(entry->name));
	memcpy(entry->name, name, len);

	HASH_ADD_STR(sym_entries, name, entry);
	set_boxed_sym(&sym, entry);

	return sym;
}

int
is_sym(sval_t v)
{
	return get_storage(v) == VAL_STORAGE_BOXED
	    && get_boxed_type(v) == VAL_BOXED_TYPE_SYM;
}

const char *
sym_name(sval_t v)
{
	assert(is_sym(v));

	struct sym_entry *entry = get_boxed_sym_ptr(v);

	return entry->name;
}
