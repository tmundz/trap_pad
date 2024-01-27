
#ifndef GUI_H
#define GUI_H

#include <SDL2/SDL.h>

class GUI {
public:
  GUI();
  ~GUI(); // destructor

  bool init();                 // Initialize the SDL window and renderer
  void run();                  // Run the main loop
  SDL_Renderer *getRenderer(); // Getter for the renderer

private:
  SDL_Window *window;
  SDL_Renderer *renderer;

  void handleEvents(); // Handle window events
  void render();       // Render the window contents
};

#endif // GUI_H
