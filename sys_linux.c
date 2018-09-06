#include <sys/types.h>
#include <sys/stat.h>
#include <sys/mman.h>

#include <assert.h>
#include <fcntl.h>
#include <stdint.h>

#include "sys.h"

const char *
sys_mmap_file_private_readonly(const char *path)
{
	int fd = open(path, O_RDONLY);
	if (fd == -1)
		return NULL;

	struct stat st;
	int rc = fstat(fd, &st);
	assert(rc != -1);

	size_t mmap_size = (size_t)st.st_size;
	assert(mmap_size < SIZE_MAX);

	void *p = mmap(NULL, mmap_size + 1, PROT_READ, MAP_PRIVATE, fd, 0);
	assert(p != MAP_FAILED);

	return (const char *)p;
}
