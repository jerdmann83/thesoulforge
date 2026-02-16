#include "raylib.h"
#include <math.h>
#include <stdio.h>

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
    Vector2 start; // Store start for drawing the trail
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

int main(void) {
    const int screenWidth = 1024;
    const int screenHeight = 768;
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
                        
                        // PLAYER SPEED (Constant but fast)
                        float angle = get_angle(siloPos, msl->target);
                        printf("%f,%f -> %f,%f -> angle is %f\n", 
                                siloPos.x,
                                siloPos.y,
                                msl->target.x,
                                msl->target.y,
                                angle);
                        msl->velocity = (Vector2){ cos(angle) * 10.0f, sin(angle) * 10.0f };
                        break;
                    }
                }
            }

            // SPAWN ENEMY MISSILES (Variable Speeds)
            if (GetRandomValue(0, 100) < 99) {
                for (int i = 0; i < MAX_ENEMIES; i++) {
                    Missile* msl = &enemyMissiles[i];
                    if (!msl->active) {
                        msl->active = true;
                        msl->start = (Vector2){ (float)GetRandomValue(0, screenWidth), 0 };
                        msl->pos = msl->start;
                        msl->color = RED;
                        msl->color_phase = (float)GetRandomValue(0, 100) * PI;

                        int targetIdx = GetRandomValue(0, MAX_CITIES);
                        msl->target = (targetIdx < MAX_CITIES) ? cities[targetIdx].pos : siloPos;
                        
                        float speed = (float)GetRandomValue(10, 35) / 10.0f;
                        float angle = get_angle(msl->pos, msl->target);
                        msl->velocity = (Vector2){ cos(angle) * speed, sin(angle) * speed };
                        printf("emissile: start %f,%f target %f,%f\n",
                                msl->start.x,
                                msl->start.y,
                                msl->target.x,
                                msl->target.y);
                        break;
                    }
                }
            }

            // UPDATE & COLLISION (Standard logic)
            for (int i = 0; i < MAX_MISSILES; i++) {
                Missile* msl = &playerMissiles[i];
                if (!msl->active) continue;

                msl->pos.x += msl->velocity.x;
                msl->pos.y += msl->velocity.y;
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
                Missile* msl = &enemyMissiles[i];
                if (!msl->active) continue;
                msl->pos.x += msl->velocity.x;
                msl->pos.y += msl->velocity.y;

                for (int j = 0; j < MAX_EXPLOSIONS; j++) {
                    if (!explosions[j].active) continue;
                    if (CheckCollisionCircles(msl->pos, 2, explosions[j].pos, explosions[j].radius)) {
                        msl->active = false;
                        score += 100;
                    }
                }

                for (int c = 0; c < MAX_CITIES; c++) {
                    City* city = &cities[c];
                    if (!city->active) continue;

                    if (CheckCollisionPointRec(msl->pos, (Rectangle){city->pos.x-15, city->pos.y-10, 30, 20})) {
                        city->active = false;
                        msl->active = false;
                    }
                }
                if (msl->pos.y > screenHeight) msl->active = false;
            }

            for (int i = 0; i < MAX_EXPLOSIONS; i++) {
                Explosion* exp = &explosions[i];
                if (exp->active) {
                    exp->radius += 1.5f;
                    if (exp->radius > exp->maxRadius) exp->active = false;
                }
            }

            int activeCities = 0;
            for (int i = 0; i < MAX_CITIES; i++) if (cities[i].active) activeCities++;
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
