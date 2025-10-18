#ifndef DOORS_LIB_FFI_RCP_HEADER
#define DOORS_LIB_FFI_RCP_HEADER

#include <stdint.h>

#define NetIp_DEFAULT_PORT 9933

typedef struct FfiBytes {
  uint64_t len;
  uint64_t capacity;
  uint64_t offset;
  uint8_t *bytes;
} FfiBytes;

/**
 * 回调用函数的返回值在dart中并不支持，所以没有返回值
 */
typedef void (*CallBack)(struct FfiBytes);

struct FfiBytes init(CallBack callback);

struct FfiBytes un_init(void);

struct FfiBytes call(const uint8_t *bytes, uint64_t length);

void bytes_free(struct FfiBytes data);

#endif  /* DOORS_LIB_FFI_RCP_HEADER */
