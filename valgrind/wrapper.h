#include <stdbool.h>
#include <stddef.h>

#include <valgrind/cachegrind.h>
#include <valgrind/callgrind.h>
#include <valgrind/dhat.h>
#include <valgrind/drd.h>
#include <valgrind/helgrind.h>
#include <valgrind/memcheck.h>
#include <valgrind/valgrind.h>

const unsigned int VALGRIND_MAJOR = __VALGRIND_MAJOR__;
const unsigned int VALGRIND_MINOR = __VALGRIND_MINOR__;

int vg_print(char *msg) { 
  return VALGRIND_PRINTF("%s", msg); 
}

int vg_print_backtrace(char *msg) {
  return VALGRIND_PRINTF_BACKTRACE("%s", msg);
}

size_t vg_do_client_request(size_t _zzq_default, size_t _zzq_request,
                            size_t _zzq_arg1, size_t _zzq_arg2,
                            size_t _zzq_arg3, size_t _zzq_arg4,
                            size_t _zzq_arg5) {
  return VALGRIND_DO_CLIENT_REQUEST_EXPR(_zzq_default, _zzq_request, _zzq_arg1,
                                         _zzq_arg2, _zzq_arg3, _zzq_arg4,
                                         _zzq_arg5);
}

{{% client_request_defs %}}
