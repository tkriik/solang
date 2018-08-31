#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "uthash.h"

#include "conf.h"
#include "val.h"

struct sym_entry {
	char		name[SYM_MAX_LEN + 1];
	UT_hash_handle	hh;
};

static struct sym_entry *sym_entries;

val_t
sym(const char *name)
{
	return symn(name, strlen(name));
}

val_t
symn(const char *name, size_t len)
{
	assert(0 < len);
	assert(len <= SYM_MAX_LEN);
	assert(HASH_COUNT(sym_entries) <= SYM_MAX_CNT);

	val_t sym = err_undef();

	struct sym_entry *old_entry;
	HASH_FIND_STR(sym_entries, name, old_entry);
	if (old_entry != NULL) {
		_set_boxed_sym(&sym, old_entry);
		return sym;
	}

	struct sym_entry *new_entry = malloc(sizeof(*new_entry));
	memset(new_entry->name, 0, sizeof(new_entry->name));
	memcpy(new_entry->name, name, len);

	HASH_ADD_STR(sym_entries, name, new_entry);
	_set_boxed_sym(&sym, new_entry);

	return sym;
}

int
is_sym(val_t v)
{
	return _get_storage(v) == VAL_STORAGE_BOXED
	    && _get_boxed_type(v) == VAL_BOXED_TYPE_SYM;
}

const char *
sym_name(val_t v)
{
	assert(is_sym(v));

	struct sym_entry *entry = _get_boxed_sym_ptr(v);
	assert(entry != NULL);

	return entry->name;
}
