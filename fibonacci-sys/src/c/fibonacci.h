#include <stdint.h>

typedef void* Fibonacci_t;

Fibonacci_t* fibo_new();
uint32_t fibo_next(Fibonacci_t* handle);

