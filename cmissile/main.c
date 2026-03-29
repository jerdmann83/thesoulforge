#include "raylib.h"
#include <math.h>
#include <stdio.h>
#include <assert.h>
#include "ent.h"

void DrawGlowLine(Vector2 start, Vector2 end, Color col) {
    Vector2 mid = { (start.x + end.x) / 2, (start.y + end.y) / 2 };
    DrawLineEx(start, end, 20.0f, Fade(col, 0.3f));
    DrawLineEx(mid, end, 1.0f, col);
}

float get_angle(Vector2 start, Vector2 end) {
    return atan2(end.y - start.y, end.x - start.x);
}

Vector2 get_velocity(float angle, float speed) {
    return (Vector2){ cos(angle) * speed, sin(angle) * speed };
}

void apply_velocity(Vector2* out, Vector2 velocity, float mult) {
    out->x += velocity.x * mult;
    out->y += velocity.y * mult;
}

const int screenWidth = 1024;
const int screenHeight = 768;
bool on_screen(Vector2 pos) {
    return pos.x >= 0 && pos.x <= screenWidth
        && pos.y >= 0 && pos.y <= screenHeight;
}

#define MAX_CITIES 6
#define MAX_MISSILES 32
#define MAX_ENEMIES 100
#define MAX_ROCKS 32
#define MAX_EXPLOSIONS 100

int get_empty_slot(ent_handle_t* handles, int len) {
    for (int i=0; i<len; ++i) {
        if (handles[i] == 0) return i;
    }
    return -1;
}

int destroy_entity(ent_context_t* ent, int idx, ent_handle_t* handles, int len) {
    assert(idx < len);
    ent_handle_t hdl = handles[idx];
    ent_destroy(ent, hdl);
    handles[idx] = 0;
}

int main(void) {
    InitWindow(screenWidth, screenHeight, "Raylib Missile Command");

    ent_context_t* ent = init_ent_context();

    ent_handle_t cities[MAX_CITIES] = { 0 };
    ent_handle_t missiles[MAX_MISSILES] = { 0 };
    ent_handle_t enemies[MAX_ENEMIES] = { 0 };
    ent_handle_t rocks[MAX_ROCKS] = { 0 };
    ent_handle_t explosions[MAX_EXPLOSIONS] = { 0 };

    int score = 0;
    bool gameOver = false;
    for (int i = 0; i < 6; i++) {
        float space = screenWidth / 7;
        Vector2 pos = (Vector2){ (i+1) * space, screenHeight - 25.0f };
        cities[i] = ent_make_city(ent, pos);
    }

    Vector2 siloPos = { screenWidth / 2, screenHeight - 30 };
    ent_handle_t silo = ent_make_silo(ent, siloPos);

    int ent_counts[eLast];

    SetTargetFPS(600);

    while (!WindowShouldClose()) {
        float speedMultiplier = 1.0;
        if (IsKeyDown(KEY_SPACE)) {
            speedMultiplier = .1;
        }

        if (!gameOver) {
            // FIRE PLAYER MISSILE
            if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
                int idx = get_empty_slot(missiles, sizeof(missiles));
                if (idx > -1) {
                    Vector2 target = GetMousePosition();
                    float angle = get_angle(siloPos, target);
                    missiles[idx] = ent_make_missile(ent,
                            siloPos,
                            target,
                            get_velocity(angle, 10.0f),
                            DARKBLUE,
                            (float)GetRandomValue(0, 100) * PI);
                }
            }

            // SPAWN ENEMY MISSILES
            if (GetRandomValue(0, 100) < 99) {
                int e = get_empty_slot(enemies, sizeof(enemies));
                if (e > -1) {
                    Vector2 target = (Vector2) { -1, -1 };
                    while (target.x < 0) {
                        int targetIdx = GetRandomValue(0, MAX_CITIES);
                        if (targetIdx == MAX_CITIES) {
                            target = siloPos;
                            continue;
                        }
                        ent_handle_t handle = cities[targetIdx];
                        Entity* city = ent_get(ent, handle);
                        if (city->active) {
                            target = city->pos;
                            continue;
                        }
                    }
                    Vector2 start = (Vector2){ 
                            (float)GetRandomValue(0, screenWidth),
                            0 };
                    float speed = (float)GetRandomValue(10, 35) / 10.f;
                    float angle = get_angle(start, target);
                    Vector2 velocity = get_velocity(angle, speed);

                    enemies[e] = ent_make_missile(ent,
                            start,
                            target,
                            velocity,
                            RED,
                            (float)GetRandomValue(0, 100) * PI);
                }
            }

            // SPAWN ROCKS
            // if (GetRandomValue(0, 99) < 100) {
            //     for (int i = 0; i < MAX_ROCKS; i++) {
            //         Rock* rock = &rocks[i];
            //         if (rock->active) continue;
            //
            //         rock->active = true;
            //         rock->radius = 30.0;
            //         rock->start = (Vector2){ GetRandomValue(0, screenWidth), 0 };
            //         rock->pos = rock->start;
            //         rock->velocity = (Vector2){ 0, 10.0 };
            //     }
            // }
            //
            // UPDATE & COLLISION
            for (int i = 0; i < MAX_ROCKS; i++) {
                // Rock* rock = &rocks[i];
                // if (!rock->active) continue;
                // apply_velocity(&rock->pos, rock->velocity, speedMultiplier);
                // if (!on_screen(rock->pos)) {
                //     rock->active = false;
                //     continue;
                // }
            }

            for (int m = 0; m < MAX_MISSILES; m++) {
                int hdl = missiles[m];
                Entity* msl = ent_get(ent, hdl);
                if (!msl->active) continue;

                apply_velocity(&msl->pos, msl->velocity, speedMultiplier);
                bool explodes = false;
                if (CheckCollisionPointCircle(msl->pos, msl->target, 5.0f)) {
                    explodes = true;
                } else { 
                    for (int m = 0; m < MAX_ROCKS; m++) {
                        // Rock* rock = &rocks[m];
                        // if (!rock->active) continue;
                        //
                        // if (CheckCollisionPointCircle(msl->pos, msl->target, 5.0f)) {
                        //     explodes = true;
                        // }
                    }
                }
                if (explodes) {
                    destroy_entity(ent, m, missiles, sizeof(missiles));

                    int idx = get_empty_slot(explosions, sizeof(explosions));
                    if (idx > -1) {
                        explosions[idx] = ent_make_explosion(ent, msl->target, 45);
                    }
                }
            }

            for (int e = 0; e < MAX_ENEMIES; e++) {
                ent_handle_t hdl_enemy = enemies[e];
                Entity* enemy = ent_get(ent, hdl_enemy);
                if (!enemy->active) continue;

                apply_velocity(&enemy->pos, enemy->velocity, speedMultiplier);
                if (e == 0) {
                    printf("enemy new pos %f,%f vel %f,%f\n", 
                            enemy->pos.x,
                            enemy->pos.y,
                            enemy->velocity.x,
                            enemy->velocity.y);
                }

                for (int ex = 0; ex < MAX_EXPLOSIONS; ex++) {
                    ent_handle_t hdl_enemy = enemies[e];
                    Entity* exp = ent_get(ent, explosions[ex]);
                    if (!exp->active) continue;

                    if (CheckCollisionCircles(enemy->pos, 2, 
                                              exp->pos, exp->radius)) {
                        destroy_entity(ent, e, enemies, sizeof(enemies));
                        score += 100;
                        break;
                    }
                }

                for (int j = 0; j < MAX_ROCKS; j++) {
                    // Rock* rock = &rocks[j];
                    // if (!rock->active) continue;
                    //
                    // if (CheckCollisionCircles(enemy->pos, 2,
                    //             rocks[j].pos, rocks[j].radius)) {
                    //     enemy->active = false;
                    //
                    //     for (int j = 0; j < MAX_EXPLOSIONS; j++) {
                    //         if (!explosions[j].active) {
                    //             explosions[j] = (Explosion){ enemy->pos, 0, 45, true };
                    //             break;
                    //         }
                    //     }
                    //     break;
                    // }
                }

                for (int c = 0; c < MAX_CITIES; c++) {
                    ent_handle_t hdl_city = cities[c];
                    Entity* city = ent_get(ent, hdl_city);
                    if (!city->active) continue;

                    if (CheckCollisionPointRec(enemy->pos, 
                                (Rectangle){city->pos.x-15, city->pos.y-10, 30, 20})) {
                        destroy_entity(ent, c, cities, sizeof(cities));
                        destroy_entity(ent, e, enemies, sizeof(enemies));
                    }
                }

                if (!on_screen(enemy->pos)) {
                    destroy_entity(ent, e, enemies, sizeof(enemies));
                    continue;
                }
            }

            for (int ex = 0; ex < MAX_EXPLOSIONS; ex++) {
                ent_handle_t hdl = explosions[ex];
                Entity* exp = ent_get(ent, hdl);
                if (exp->active) {
                    exp->radius += 1.5f;
                    if (exp->radius > exp->max_radius) {
                        destroy_entity(ent, ex, explosions, sizeof(explosions));
                    }
                }
            }

            int activeCities = 0;
            for (int i = 0; i < MAX_CITIES; i++) {
                Entity* city = ent_get(ent, cities[i]);
                if (city->active) {
                    activeCities++;
                }
            }

            // if (activeCities == 0) gameOver = true;

        } else if (IsKeyPressed(KEY_R)) { 
            gameOver = false; 
            score = 0;
            assert(ent_get_count(ent, eCity) == 0);
            for (int i = 0; i < MAX_CITIES; i++) {
                // cities[i].active = true;
            }
        }

        // DRAW
        BeginDrawing();
            ClearBackground(BLACK);
            
            if (!gameOver) {
                for (int i = 0; i < MAX_CITIES; i++) {
                    Entity* city = ent_get(ent, cities[i]);
                    if (city->active) {
                        DrawRectangle(city->pos.x - 15, city->pos.y - 10, 30, 20, DARKBLUE);
                    }
                }
                DrawPoly(siloPos, 3, 30, 0, DARKGRAY);
                
                for (int i = 0; i < MAX_ENEMIES; i++) {
                    Entity* enemy = ent_get(ent, enemies[i]);
                    if (enemy->active) {
                        if (i == 0)
                        printf("enemy hdl=%i act=%i\n", 
                                enemies[i], enemy->active);
                        float fade = sin(enemy->color_phase) * .2;
                        DrawLineEx(enemy->start, enemy->target,
                                20.0f, Fade(enemy->color, .1));
                        enemy->color_phase += PI / 60.;
                        DrawLineEx(enemy->start, enemy->pos,
                                1.0f, Fade(enemy->color, .8 + fade));
                    }
                }

                for (int i = 0; i < MAX_MISSILES; i++) {
                    Entity* msl = ent_get(ent, missiles[i]);
                    if (msl->active) {
                        DrawGlowLine(msl->start, msl->pos, msl->color);
                    }
                }

                for (int i = 0; i < MAX_EXPLOSIONS; i++) {
                    Entity* exp = ent_get(ent, explosions[i]);
                    if (exp->active) {
                        DrawCircleV(exp->pos, exp->radius, Fade(WHITE, 0.4f));
                        DrawCircleLines(exp->pos.x, exp->pos.y, exp->radius, GOLD);
                    }
                }

                for (int i = 0; i < MAX_ROCKS; i++) {
                    Entity* rock = ent_get(ent, rocks[i]);
                    if (rock->active) {
                        DrawCircleV(rock->pos, rock->radius, Fade(GRAY, 0.4f));
                        DrawCircleLines(rock->pos.x, rock->pos.y, rock->radius, GRAY);
                    }
                }

                DrawText(TextFormat("SCORE: %06i", score), 10, 10, 20, RAYWHITE);
            } else {
                DrawText("GAME OVER", screenWidth/2 - 100, screenHeight/2 - 20, 40, RED);
                DrawText("PRESS [R] TO RESTART", screenWidth/2 - 90, screenHeight/2 + 60, 10, GRAY);
            }
        EndDrawing();
    }
    CloseWindow();
    return 0;
}
