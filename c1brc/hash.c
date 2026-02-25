#include "hash.h"
#include "hash_funcs.h"
#include "assert.h"
#include "string.h"
#include "stdlib.h"
#include "stdio.h"
#include "stdbool.h"

typedef struct {
    char name[24];
} loc_names;

typedef struct {
    val_t min;
    val_t max;
    val_t tot;
    unsigned long num;
} loc_stats;

typedef unsigned long (*hash_func_t)(const char*);
enum hash_func {
    hash_func__djb2,
    hash_func__fnv1a,
    hash_func__sdbm,
    hash_func__last,
};
struct hash_context {
    loc_names* names;
    loc_stats* stats;
    unsigned long num_slots;

    hash_func_t hash_funcs[hash_func__last];
    unsigned long hash_func_hits[hash_func__last];
};

bool is_slot_usable(const loc_names* names, const char* loc) {
    if (names->name[0] == 0) return true;
    if (strcmp(names->name, loc) == 0) return true;

    return false;
}

int hash_slot_from_hash(hash_context_t* ctx, unsigned long hash) {
    int usable_slots = ctx->num_slots - 1;
    return (hash % usable_slots) + 1;
}

int hash_get_usable_slot(hash_context_t* ctx, const char* loc) {

    for (unsigned long i=0; i<hash_func__last; ++i) {
        ++ctx->hash_func_hits[i];

        hash_func_t func = ctx->hash_funcs[i];
        int slot = hash_slot_from_hash(ctx, func(loc));

        const loc_names* names = &ctx->names[slot];
        if (is_slot_usable(&names[slot], loc)) return slot;
    }

    return 0;
}

unsigned long hash_get_slot(hash_context_t* ctx, const char* loc) {
    int slot = hash_get_usable_slot(ctx, loc);
    if (slot < 1) {
        assert(!"no slots!");
        return 0;
    }

    loc_names* names = &ctx->names[slot];
    unsigned long len = sizeof(names->name);
    unsigned long input_len = strlen(loc);
    if (input_len < len) len = input_len;
    strncpy(names->name, loc, len);
    return slot;
}

void hash_on_val(hash_context_t* ctx, 
        const char* loc, val_t val) {
    assert(val < 1000.0);
    assert(val > -1000.0);
    unsigned long slot = hash_get_slot(ctx, loc);
    if (slot < 1) {
        assert(false);
        return;
    }

    loc_stats* stats = &ctx->stats[slot];
    if (val < stats->min) stats->min = val;
    if (val > stats->max) stats->max = val;
    stats->tot += val;
    stats->num += 1;
}

void hash_print_slots(hash_context_t* ctx) {
    unsigned long slots_used = 0;
    for (unsigned long i=0; i<ctx->num_slots; ++i) {
        if (ctx->names[i].name[0] == 0) continue;

        ++slots_used;
        loc_names* ln = &ctx->names[i];
        loc_stats* ls = &ctx->stats[i];
        assert(ls->tot < 1000000);
        assert(ls->num < 1000000);
        assert((val_t)ls->tot / (val_t)ls->num < 1000000.0);
        assert((val_t)ls->tot / (val_t)ls->num > -1000000.0);
        printf("#%lu \"%s\" => samples:%lu min:%f max:%f avg:%f\n",
                i,
                ln->name,
                ls->num,
                ls->min,
                ls->max,
                (val_t)ls->tot / (val_t)ls->num);
    }
    printf("\n%lu of %lu slots used", 
            slots_used, 
            ctx->num_slots);
    printf("hits:");
    for (unsigned long i=0; i<hash_func__last; ++i) {
        printf(" %lu", ctx->hash_func_hits[i]);
    }
    printf("\n");
}

hash_context_t* hash_context_init(unsigned long num) {
    hash_context_t* ctx = malloc(sizeof(hash_context_t));
    memset(ctx, 0, sizeof(*ctx));

    if (num < 4) num = 4;

    unsigned long stats_len = sizeof(loc_stats) * num;
    ctx->stats = malloc(stats_len);
    memset(ctx->stats, 0, stats_len);

    unsigned long names_len = sizeof(loc_names) * num;
    ctx->names = malloc(names_len);
    memset(ctx->names, 0, names_len);

    ctx->hash_funcs[hash_func__djb2] = hash_djb2;
    ctx->hash_funcs[hash_func__fnv1a] = hash_fnv1a;
    ctx->hash_funcs[hash_func__sdbm] = hash_sdbm;

    memset(ctx->hash_func_hits, 0, sizeof(ctx->hash_func_hits));

    ctx->num_slots = num;
    return ctx;
}
