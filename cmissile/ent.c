#include "ent.h"

#include <assert.h>
#include <raylib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ENTITIES_LEN 256
struct ent_context {
    Entity entities[ENTITIES_LEN];
    size_t entities_len;
    int next_handle;
    int num[eLast];
};

const Vector2 nullvec = (Vector2) { 0.0, 0.0 };

const ent_handle_t ent_null_handle = 0;

int get_entity_handle(ent_context_t* ent) {
    for (int i=1; i<ENTITIES_LEN; ++i) {
        if (ent->entities[i].active) continue;
        assert(i > ent_null_handle);
        return i;
    }
    return 0;
}

typedef struct {
    ent_handle_t hdl;
    Entity* ent;
} slot_t;

int ent_find_slot(ent_context_t* ent, int hdl_start) {
    for (int cur=0; cur<ENTITIES_LEN; ++cur) {
        int hdl = (hdl_start + cur) % ENTITIES_LEN;
        if (!ent->entities[hdl].active) {
            ent->next_handle = (hdl + 1) % ENTITIES_LEN;
            if (ent->next_handle == 0) ent->next_handle = 1;

            return hdl;
        }
    }
    return 0;
}

slot_t ent_get_slot(ent_context_t* ent) {
    int hdl = ent_find_slot(ent, ent->next_handle);
    Entity* e = &ent->entities[hdl];
    if (e->active) {
        assert(hdl > 0);
    }
    return (slot_t) {
        .ent = e,
        .hdl = hdl,
    };
}

int ent_on_make(ent_context_t* ent, slot_t slot, int mod) {
    assert(slot.ent->type < sizeof(ent->entities));
    ent->num[slot.ent->type] += mod;
    printf("make. hdl=%i type=%i mod=%i act=%i\n", 
            slot.hdl, slot.ent->type, mod, slot.ent->active);
}

ent_handle_t ent_make_city(void* ctx, Vector2 pos) {
    ent_context_t* ent = (ent_context_t*)ctx;
    slot_t slot = ent_get_slot(ent);
    if (slot.hdl > 0) {
        *slot.ent = (Entity) {
            .type = eCity, 
            .active = true,
            .pos = pos, 
            .start = pos,
            .target = pos,
            .velocity = (Vector2){ 0.0, 0.0 },
            .color = DARKBLUE,
            .color_phase = 0.0,
            .radius = 0,
            .max_radius = 0,
        };
        ent_on_make(ent, slot, 1);
    }
    return slot.hdl;
}

ent_handle_t ent_make_silo(void* ctx, Vector2 pos) {
    ent_context_t* ent = (ent_context_t*)ctx;
    slot_t slot = ent_get_slot(ent);
    if (slot.hdl > 0) {
        *slot.ent = (Entity) {
            .type = eSilo,
            .active = true,
            .pos = pos, 
            .start = pos,
            .target = pos,
            .velocity = (Vector2){ 0.0, 0.0 },
            .color = DARKBLUE,
            .color_phase = 0.0,
            .radius = 0,
            .max_radius = 0,
        };
        ent_on_make(ent, slot, 1);
    }
    return slot.hdl;
}

ent_handle_t ent_make_missile(void* ctx, 
        Vector2 start, 
        Vector2 target, 
        Vector2 velocity,
        Color color,
        float color_phase) {
    ent_context_t* ent = (ent_context_t*)ctx;
    slot_t slot = ent_get_slot(ent);
    if (slot.hdl > 0) {
        *slot.ent = (Entity) {
            .type = eMissile, 
            .active = true,
            .pos = start, 
            .start = start,
            .target = target,
            .velocity = velocity,
            .color = color,
            .color_phase = color_phase,
            .radius = 0,
            .max_radius = 0,
        };
        ent_on_make(ent, slot, 1);
    }
    return slot.hdl;
}

ent_handle_t ent_make_rock(void* ctx, 
        Vector2 pos, 
        Vector2 velocity, 
        float mass) {
    ent_context_t* ent = (ent_context_t*)ctx;
    slot_t slot = ent_get_slot(ent);
    if (slot.hdl > 0) {
        *slot.ent = (Entity) {
            .type = eRock, 
            .active = true,
            .pos = pos, 
            .start = pos,
            .target = pos,
            .velocity = velocity,
            .color = GRAY,
            .color_phase = 0.0,
            .radius = 0,
            .max_radius = 0,
        };
        ent_on_make(ent, slot, 1);
    }
    return slot.hdl;
}

ent_handle_t ent_make_explosion(void* ctx, Vector2 pos, float max_radius) {
    ent_context_t* ent = (ent_context_t*)ctx;
    slot_t slot = ent_get_slot(ent);
    if (slot.hdl > 0) {
        *slot.ent = (Entity) {
            .type = eExplosion, 
            .active = true,
            .pos = pos, 
            .start = nullvec,
            .target = nullvec,
            .velocity = nullvec,
            .color = DARKBLUE,
            .color_phase = 0.0,
            .radius = 0,
            .max_radius = max_radius,
        };
        ent_on_make(ent, slot, 1);
    }
    return slot.hdl;
}

ent_context_t* init_ent_context() {
    ent_context_t* ctx = malloc(sizeof(ent_context_t));
    memset(ctx, 0, sizeof(*ctx));

    ctx->entities[0] = (Entity) {
            .type = eNoEntity, 
            .pos = nullvec,
            .active = false,
            .start = nullvec,
            .target = nullvec,
            .velocity = nullvec,
            .color = DARKBLUE,
            .color_phase = 0.0,
            .radius = 0,
            .max_radius = 0,
    };
    ctx->next_handle = 1;
    return ctx;
}

int ent_get_count(ent_context_t* ctx, int type) {
    assert(type < eLast);
    return ctx->num[type];
}

Entity* ent_get(ent_context_t* ctx, ent_handle_t hdl) {
    assert(hdl < sizeof(ctx->entities));
    return &ctx->entities[hdl];
}

void ent_destroy(ent_context_t* ctx, ent_handle_t hdl) {
    assert(hdl < sizeof(ctx->entities));
    ctx->entities[hdl].active = false;

    slot_t slot = {
        .hdl = hdl,
        .ent = &ctx->entities[hdl],
    };
    ent_on_make(ctx, slot, -1);
}
