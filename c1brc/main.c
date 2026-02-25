#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "hash.h"

void impl_c(const char* fname) {
    FILE* fp = fopen(fname, "r");
    if (!fp) abort();

    hash_context_t* ctx = hash_context_init(1 * 1024 * 1024);
    char* line = NULL;
    size_t len = 0;
    ssize_t nread = 0;

    while ((nread = getline(&line, &len, fp)) != -1) {
        size_t l1, l2;
        char* cloc;
        char* cval;
        // Meoqui;55.4
        cloc = strtok(line, ";");
        cval = strtok(NULL, ";");
        val_t val = atof(cval);
        hash_on_val(ctx, cloc, val);
    }
    hash_print_slots(ctx);
}

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "expect: [fname]\n");
        return -1;
    }
    impl_c(argv[1]);
}
