#ifndef EDITOR_H
#define EDITOR_H

#include <SDL2/SDL.h>
#include <string>
#include <vector>

class Editor {
public:
  Editor();
  void handleEvent(SDL_Event &e);

  // Editor methods like cursor movement, text input, etc.

private:
  // Cursor position, text lines, etc.
};

#endif // EDITOR_H
