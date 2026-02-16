#include "raylib.h"
#include <math.h>
#include <stdio.h>
#include <assert.h>

#define MAX_MISSILES 4
#define MAX_EXPLOSIONS 20
#define MAX_ENEMIES 10
#define MAX_CITIES 6

// gravity!  missiles and rocks need mass
// unrelated: 1brc with hashmap with chaining impl

typedef struct { 
    Vector2 pos; 
    bool active; 
} City;
typedef struct { 
    Vector2 start;
    Vector2 pos; 
    Vector2 target; 
    Vector2 velocity; 
    Color color; 
    float color_phase;
    bool active; 
} Missile;

typedef struct { 
    Vector2 pos; 
    float radius; 
    float maxRadius; 
    bool active; 
} Explosion;

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

int main(void) {
    InitWindow(screenWidth, screenHeight, "Raylib Neon Missile Command");

    int score = 0;
    bool gameOver = false;
    City cities[MAX_CITIES];
    for (int i = 0; i < MAX_CITIES; i++) {
        cities[i] = (City){ 
            (Vector2){ 100.0f + i * 120.0f, screenHeight - 25.0f }, 
                true };
    }

    Vector2 siloPos = { screenWidth / 2, screenHeight - 30 };
    Missile playerMissiles[MAX_MISSILES] = { 0 };
    Missile enemyMissiles[MAX_ENEMIES] = { 0 };
    Explosion explosions[MAX_EXPLOSIONS] = { 0 };

    // Possible trail colors
    Color neonColors[] = { RED, MAGENTA, LIME, SKYBLUE, ORANGE, VIOLET };

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        float speedMultiplier = 1.0;
        if (IsKeyDown(KEY_SPACE)) {
            speedMultiplier = .1;
        }

        if (!gameOver) {
            // FIRE PLAYER MISSILE
            if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
                for (int i = 0; i < MAX_MISSILES; i++) {
                    Missile* msl = &playerMissiles[i];
                    if (!msl->active) {
                        msl->active = true;
                        msl->start = siloPos;
                        msl->pos = siloPos;
                        msl->target = GetMousePosition();
                        msl->color = SKYBLUE;
                        
                        float angle = get_angle(siloPos, msl->target);
                        msl->velocity = get_velocity(angle, 10.0f);
                        break;
                    }
                }
            }

            // SPAWN ENEMY MISSILES
            if (GetRandomValue(0, 100) < 99) {
                for (int i = 0; i < MAX_ENEMIES; i++) {
                    Missile* msl = &enemyMissiles[i];
                    if (!msl->active) {
                        msl->active = true;
                        msl->start = (Vector2){ (float)GetRandomValue(0, screenWidth), 0 };
                        msl->pos = msl->start;

                        msl->color = RED;
                        msl->color_phase = (float)GetRandomValue(0, 100) * PI;

                        msl->target = (Vector2){ -1, -1 };
                        while (msl->target.x < 0) {
                            int targetIdx = GetRandomValue(0, MAX_CITIES);
                            if (targetIdx == MAX_CITIES) {
                                msl->target = siloPos;
                                continue;
                            }
                            City* city = &cities[targetIdx];
                            if (city->active) {
                                msl->target = city->pos;
                                continue;
                            }
                        }
                        
                        float speed = (float)GetRandomValue(10, 35) / 10.f;
                        float angle = get_angle(msl->pos, msl->target);
                        msl->velocity = get_velocity(angle, speed);
                        break;
                    }
                }
            }


            // UPDATE & COLLISION
            for (int i = 0; i < MAX_MISSILES; i++) {
                Missile* msl = &playerMissiles[i];
                if (!msl->active) continue;

                apply_velocity(&msl->pos, msl->velocity, speedMultiplier);
                if (CheckCollisionPointCircle(msl->pos, msl->target, 5.0f)) {
                    msl->active = false;
                    for (int j = 0; j < MAX_EXPLOSIONS; j++) {
                        if (!explosions[j].active) {
                            explosions[j] = (Explosion){ msl->target, 0, 45, true };
                            break;
                        }
                    }
                }
            }

            for (int i = 0; i < MAX_ENEMIES; i++) {
                Missile* enemy = &enemyMissiles[i];
                if (!enemy->active) continue;

                apply_velocity(&enemy->pos, enemy->velocity, speedMultiplier);

                for (int j = 0; j < MAX_EXPLOSIONS; j++) {
                    if (!explosions[j].active) continue;
                    if (CheckCollisionCircles(enemy->pos, 2, explosions[j].pos, explosions[j].radius)) {
                        enemy->active = false;
                        score += 100;
                    }
                }

                for (int c = 0; c < MAX_CITIES; c++) {
                    City* city = &cities[c];
                    if (!city->active) continue;

                    if (CheckCollisionPointRec(enemy->pos, 
                                (Rectangle){city->pos.x-15, city->pos.y-10, 30, 20})) {
                        city->active = false;
                        enemy->active = false;
                    }
                }
                if (enemy->pos.y > screenHeight) {
                    enemy->active = false;
                    continue;
                }
            }

            for (int i = 0; i < MAX_EXPLOSIONS; i++) {
                Explosion* exp = &explosions[i];
                if (exp->active) {
                    exp->radius += 1.5f;
                    if (exp->radius > exp->maxRadius) exp->active = false;
                }
            }

            int activeCities = 0;
            for (int i = 0; i < MAX_CITIES; i++) {
                if (cities[i].active) {
                    activeCities++;
                }
            }

            // if (activeCities == 0) gameOver = true;

        } else if (IsKeyPressed(KEY_R)) { 
            gameOver = false; 
            score = 0;
            for (int i = 0; i < MAX_CITIES; i++) cities[i].active = true;
        }

        // DRAW
        BeginDrawing();
            ClearBackground(BLACK);
            
            if (!gameOver) {
                for (int i = 0; i < MAX_CITIES; i++) {
                    if (cities[i].active) {
                        DrawRectangle(cities[i].pos.x - 15, cities[i].pos.y - 10, 30, 20, DARKBLUE);
                    }
                }
                DrawPoly(siloPos, 3, 30, 0, DARKGRAY);
                
                for (int i = 0; i < MAX_ENEMIES; i++) {
                    if (enemyMissiles[i].active) {
                        float fade = sin(enemyMissiles[i].color_phase) * .2;
                        DrawLineEx(enemyMissiles[i].start, enemyMissiles[i].target,
                                20.0f, Fade(enemyMissiles[i].color, .1));
                        enemyMissiles[i].color_phase += PI / 60.;
                        DrawLineEx(enemyMissiles[i].start, enemyMissiles[i].pos,
                                1.0f, Fade(enemyMissiles[i].color, .8 + fade));
                    }
                }

                for (int i = 0; i < MAX_MISSILES; i++) {
                    if (playerMissiles[i].active) {
                        DrawGlowLine(playerMissiles[i].start, playerMissiles[i].pos, playerMissiles[i].color);
                    }
                }

                for (int i = 0; i < MAX_EXPLOSIONS; i++) {
                    Explosion* exp = &explosions[i];
                    if (exp->active) {
                        DrawCircleV(exp->pos, exp->radius, Fade(WHITE, 0.4f));
                        DrawCircleLines(exp->pos.x, exp->pos.y, exp->radius, GOLD);
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
