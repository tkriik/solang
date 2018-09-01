#ifndef SOLANG_H
#define SOLANG_H

#include <stdint.h>
#include <limits.h>

#define SOLANG_VSN_MAJOR 0
#define SOLANG_VSN_MINOR 1
#define SOLANG_VSN_PATCH 0

#if ULONG_MAX == UINT64_MAX
#define SOLANG_64BIT
#elif ULONG_MAX == UINT32_MAX
#define SOLANG_32BIT
#else
#error "unknown architecture"
#endif

#endif
