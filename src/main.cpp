#include "gui.h"

int main(int argc, char *argv[]) {
  GUI gui;
  if (!gui.init()) {
    return -1;
  }

  gui.run();

  return 0;
}
