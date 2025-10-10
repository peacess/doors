#ifndef DOORS_LIB_FFI_RCP_HEADER
#define DOORS_LIB_FFI_RCP_HEADER

#include <stdint.h>

typedef struct Bytes {
  uint64_t len;
  uint64_t capacity;
  uint64_t offset;
  uint8_t *bytes;
} Bytes;

/**
 * 回调用函数的返回值在dart中并不支持，所以没有返回值
 */
typedef void (*CallBack)(struct Bytes);

struct Bytes init(void);

struct Bytes un_init(void);

struct Bytes call(uint64_t method_idd, const struct Bytes *in_parameter);

int32_t bytes_free(struct Bytes bytes);

/**
 * if the parameter call_back is null, then cancel the callback
 */
int32_t set_call_back(CallBack call_back);

#endif  /* DOORS_LIB_FFI_RCP_HEADER */
