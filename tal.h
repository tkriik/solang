#ifndef TAL_H
#define TAL_H

#include <stdint.h>
#include <limits.h>

#define VSN_MAJOR 0
#define VSN_MINOR 1
#define VSN_PATCH 0

#if ULONG_MAX == UINT64_MAX
#define TAL_64BIT
#elif ULONG_MAX == UINT32_MAX
#define TAL_32BIT
#else
#error "unknown architecture"
#endif

#endif
