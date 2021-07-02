#include <assert.h>
#include <ctype.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// todo: why does sizeof(struct) with a blah[] member not count said member?
// the below struct is apparantly 16 bytes.  changing blah[] to blah* makes it
// 24 as I would expect, eg ptr == 8 bytes on 64-bit os.
//
// clearly a learning opportunity here...
typedef struct fatptr_s {
    size_t len;
    size_t free;
    char buf[];
} fatptr_t;

char* fatptr_new(char* s) {
    fatptr_t* sh;
    size_t len = strlen(s);
    sh = calloc(len, sizeof(fatptr_t) + (sizeof(char) * len));
    if (!sh) return NULL;

    if (!memcpy(sh->buf, s, len)) return NULL;
    sh->len = len;
    sh->free = 0;
    return sh->buf;
}

void fatptr_free(char* s) {
    free(s - sizeof(fatptr_t));
}

int main() {
    printf("%lu\n", sizeof(struct fatptr_s));
    printf("%lu\n", sizeof(fatptr_t));
    char* fp = fatptr_new("hello world!");
    printf("%s\n", fp);

    fatptr_t* real_fp = fp - sizeof(fatptr_t);
    printf("%lu %lu \"%s\"\n", 
        real_fp->len, real_fp->free, real_fp->buf);

    fatptr_free(fp);
}
