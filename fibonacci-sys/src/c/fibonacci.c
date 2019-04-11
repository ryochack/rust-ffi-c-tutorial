#include <stdlib.h>
#include "fibonacci.h"

struct Fibonacci {
    int32_t  n;
    uint32_t prev;
    uint32_t cur;
};

Fibonacci_t* fibo_new() {
    struct Fibonacci *self = calloc(1, sizeof(struct Fibonacci));
    return (void*)self;
}

uint32_t fibo_next(Fibonacci_t* handle) {
    if (!handle) {
        return 0;
    }

    struct Fibonacci *self = (struct Fibonacci*)handle;
    self->n++;
    switch (self->n) {
        case 1:
            return 0;
        case 2:
            self->cur = 1;
            return 1;
        default:
            ;
            uint32_t next = self->prev + self->cur;
            self->prev = self->cur;
            self->cur  = next;
            return next;
    }
}

