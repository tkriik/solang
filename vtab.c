#include <assert.h>
#include <stdlib.h>

#include "val.h"
#include "vtab.h"

void
vtab_init(struct vtab *vtab)
{
	assert(vtab != NULL);

	vtab->count = 0;
	for (size_t i = 0; i < VTAB_MAX_ENTRIES; i++) {
		struct vtab_entry *entry = &vtab->entries[i];
		entry->sym = err_undef();
		entry->v = err_undef();
	};
}

val_t
vtab_insert(struct vtab *vtab, val_t sym, val_t v)
{
	assert(vtab != NULL);
	assert(is_sym(sym));

	size_t i;
	for (i = 0; i < vtab->count + 1; i++) {
		struct vtab_entry *entry = &vtab->entries[i];
		if (is_eq(sym, entry->sym))
			return err_undef();

		if (is_err_undef(entry->sym)) {
			entry->sym = sym;
			entry->v = v;
			vtab->count++;

			return sym;
		}
	}

	assert(i < VTAB_MAX_ENTRIES);

	return err_undef();

}

val_t
vtab_lookup(struct vtab *vtab, val_t sym)
{
	assert(vtab != NULL);
	assert(is_sym(sym));

	for (size_t i = 0; i < vtab->count; i++) {
		struct vtab_entry *entry = &vtab->entries[i];
		if (!is_eq(sym, entry->sym))
			continue;

		return entry->v;
	}

	return err_undef();
}
