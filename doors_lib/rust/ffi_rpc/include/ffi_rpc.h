#ifndef DOORS_LIB_FFI_RCP_HEADER
#define DOORS_LIB_FFI_RCP_HEADER

#include <stdint.h>

typedef struct Bytes {
  uint8_t *bytes;
  uint64_t len;
  uint64_t capacity;
} Bytes;

struct Bytes call(uint64_t method_idd, const struct Bytes *_in_parameter);

void bytes_free(struct Bytes *bytes);

#endif  /* DOORS_LIB_FFI_RCP_HEADER */
