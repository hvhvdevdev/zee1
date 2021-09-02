#include <GL/gl.h>
#include <SDL2/SDL.h>
#include <stdbool.h>

/*
 * ─── FUNCTION: INITIALIZE VIDEO ─────────────────────────────────────────────────
 */

bool Zee1_InitVideo() {
    /*
     * ─── INIT SDL VIDEO SUBSYSTEM ────────────────────────────────────
     */

    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        return false;
    }

    /*
     * ─── OPENGL VERSION ─────────────────────────────────────────────────────────────
     */

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1);

    /*
     * ─── CREATE THE WINDOW ──────────────────────────────────────────────────────────
     */

    SDL_Window* window = SDL_CreateWindow("Zee1", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                                          1024, 768, SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL);

    /*
     * ─── SUCCESS ? ──────────────────────────────────────────────────────────────────
     */

    if (window == NULL) {
        return false;
    }

    /*
     * ─── CREATE OPENGL CONTEXT ──────────────────────────────────────────────────────
     */

    SDL_GLContext context = SDL_GL_CreateContext(window);

    /*
     * ─── SUCCESS? ───────────────────────────────────────────────────────────────────
     */

    if (context == NULL) {
        return false;
    }

    /*
     * ─── USE VSYNC ──────────────────────────────────────────────────────────────────
     */

    if (SDL_GL_SetSwapInterval(1) < 0) {
        return false;
    }

    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);

    glBegin(GL_TRIANGLES);
    glColor3f(1.0f, 1.0f, 0.0f);
    glVertex2f(0.0f, 0.0f);
    glVertex2f(1.0f, 0.0f);
    glVertex2f(0.0f, 1.0f);
    glEnd();

    SDL_GL_SwapWindow(window);

    SDL_Delay(2000);
}

/*
 * ─── FUNCTION: DELAY MS ─────────────────────────────────────────────────────────
 */

void Zee1_Delay(uint32_t ms) { SDL_Delay(ms); }

/* ──────────────────────────────────────────────────────────────────────────────── */