#include <stdint.h>

double pi_c(uint64_t iterations) {
    double pi = 3.0;
    double div = 2.0;

    for (uint64_t i = 0; i < iterations; i++) {
        pi += 4.0 / (div * (div + 1.0) * (div + 2.0));
        div += 2;
        pi -= 4.0 / (div * (div + 1.0) * (div + 2.0));
        div += 2;
    }

    return pi;
}