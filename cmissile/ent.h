#pragma once
#include "raylib.h"

enum EntityType {
    eNoEntity,
    eSilo,
    eCity,
    eMissile,
    eExplosion,
    eRock,
    eLast,
};

enum Owner {
    eNoOwner,
    ePlayer,
};

typedef struct { 
    int type;
    bool active; 
    Vector2 pos; 
    Vector2 start;
    Vector2 target; 
    Vector2 velocity; 
    // probably some super-cool way to have my own color type
    // and avoid dependency on raylib for this generic ent system
    // the thing is... I just sort of don't care right now :) 
    Color color; 
    float color_phase;
    float radius; 
    float max_radius; 
} Entity;

struct ent_context;
typedef struct ent_context ent_context_t;
ent_context_t* init_ent_context();

typedef int ent_handle_t;
ent_handle_t ent_make_silo(void* ctx, Vector2 pos);
ent_handle_t ent_make_city(void* ctx, Vector2 pos);
ent_handle_t ent_make_missile(void* ctx,
        Vector2 start, 
        Vector2 target, 
        Vector2 velocity,
        Color color,
        float color_phase);
ent_handle_t ent_make_rock(void* ctx, 
        Vector2 pos, 
        Vector2 velocity, 
        float mass);
ent_handle_t ent_make_explosion(void* ctx, 
        Vector2 pos, 
        float max_radius);

Entity* ent_get(ent_context_t* ctx, ent_handle_t handle);
int ent_get_count(ent_context_t* ctx, int type);

void ent_destroy(ent_context_t* ctx, ent_handle_t handle);
