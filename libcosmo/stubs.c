#include "cosmopolitan.h"

typedef enum {
  _URC_NO_REASON = 0,
  _URC_FOREIGN_EXCEPTION_CAUGHT = 1,
  _URC_FATAL_PHASE2_ERROR = 2,
  _URC_FATAL_PHASE1_ERROR = 3,
  _URC_NORMAL_STOP = 4,
  _URC_END_OF_STACK = 5,
  _URC_HANDLER_FOUND = 6,
  _URC_INSTALL_CONTEXT = 7,
  _URC_CONTINUE_UNWIND = 8
} _Unwind_Reason_Code;

#define UNW_TDEP_CURSOR_LEN 127

typedef uint64_t unw_word_t;

typedef struct unw_cursor {
  unw_word_t opaque[UNW_TDEP_CURSOR_LEN];
} unw_cursor_t;

struct _Unwind_Context {
  unw_cursor_t cursor;
  int end_of_stack;
};

typedef _Unwind_Reason_Code (*_Unwind_Trace_Fn)(struct _Unwind_Context *,
                                                void *);

uintptr_t _Unwind_GetCFA(struct _Unwind_Context *ctx) {
  return 0;
}

uintptr_t _Unwind_GetIP(struct _Unwind_Context *ctx) {
  return 0;
}

_Unwind_Reason_Code _Unwind_Backtrace(_Unwind_Trace_Fn fn, void *arg) {
  return _URC_NORMAL_STOP;
}

void *_Unwind_FindEnclosingFunction(void *arg) {
  return NULL;
}

#define MAX_KEYS 64

typedef unsigned pthread_key_t;
typedef void (*dtor)(void*);

typedef struct {
    pthread_key_t kval;
    dtor d;
    void *value;
} tls_t;

static tls_t my_tls[MAX_KEYS] = {0};
static unsigned thread_avail = 0;

int pthread_key_create(pthread_key_t *k, dtor destructor) {
    for(pthread_key_t i=1; i<MAX_KEYS; i++) {
        if(my_tls[i].kval == 0) {
               *k = i;
               my_tls[i].kval = i;
               my_tls[i].d = destructor;
               return 0;
        }
    }
    return -1;
}

void* pthread_getspecific(pthread_key_t k) {
    if(k < MAX_KEYS) {
        return my_tls[k].value;
    }
    return NULL;
}

int pthread_setspecific(pthread_key_t k, const void* value) {
    if(k < MAX_KEYS) {
        my_tls[k].value = value;
        return 0;
    }
    return -1;
}

int pthread_key_delete(pthread_key_t k) {
    if (k < MAX_KEYS) {
        my_tls[k].kval = 0;
        return 0;
    }
    return -1;
}

int __xpg_strerror_r(int a, char *b, size_t c) {
    return strerror_r(a, b, c);
}
