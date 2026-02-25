#include "hash_funcs.h"

unsigned long hash_fnv1a(const char *str) {
    unsigned long fnv_offset_basis = 2166136261U;
    unsigned long fnv_prime = 16777619U;

    unsigned long hash = fnv_offset_basis;
    
    while (*str) {
        hash ^= (unsigned short)*str++;
        hash *= fnv_prime;
    }
    
    return hash;
}

unsigned long hash_djb2(const char *str) {
    unsigned long hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

unsigned long hash_sdbm(const char *str) {
    unsigned long hash = 0;
    int c;

    while ((c = *str++)) {
        hash = c + (hash << 6) + (hash << 16) - hash;
    }

    return hash;
}
