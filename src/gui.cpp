#include "gui.h"

GUI::GUI() : window(nullptr), renderer(nullptr) {}

GUI::~GUI() {
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
}

bool GUI::init() {
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    SDL_Log("Unable to initialize SDL: %s", SDL_GetError());
    return false;
  }

  window = SDL_CreateWindow("Text Editor", SDL_WINDOWPOS_CENTERED,
                            SDL_WINDOWPOS_CENTERED, 800, 600, SDL_WINDOW_SHOWN);
  if (!window) {
    SDL_Log("Unable to create window: %s", SDL_GetError());
    return false;
  }

  renderer = SDL_CreateRenderer(
      window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
  if (!renderer) {
    SDL_Log("Unable to create renderer: %s", SDL_GetError());
    return false;
  }

  return true;
}

void GUI::run() {
  bool quit = false;
  SDL_Event e;

  while (!quit) {
    while (SDL_PollEvent(&e) != 0) {
      if (e.type == SDL_QUIT) {
        quit = true;
      }
      handleEvents();
    }

    render();
  }
}

void GUI::handleEvents() {
  // Handle GUI-related events here
}

void GUI::render() {
  SDL_SetRenderDrawColor(renderer, 83, 86, 127, 255); // Black background
  SDL_RenderClear(renderer);

  // Additional rendering can be done here

  SDL_RenderPresent(renderer);
}

SDL_Renderer *GUI::getRenderer() { return renderer; }
