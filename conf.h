#ifndef CONF_H
#define CONF_H

/* Maximum number of symbol-value entries per environment value table. */
#define VTAB_MAX_ENTRIES	1024

/* Symbol length limit (not including null terminator) */
#define SYM_MAX_LEN		255

/* Maximun number of symbols */
#define SYM_MAX_CNT		(1 << 20)

#endif
