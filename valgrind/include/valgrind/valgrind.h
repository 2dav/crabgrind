#ifndef __VALGRIND_H
#define __VALGRIND_H

#define __VALGRIND_MAJOR__    0
#define __VALGRIND_MINOR__    0

static int VALGRIND_PRINTF(const char *format, ...) {
  (void)format;
  return 0;
}

static int VALGRIND_PRINTF_BACKTRACE(const char *format, ...) {
  (void)format;
  return 0;
}

#endif // __VALGRIND_H
