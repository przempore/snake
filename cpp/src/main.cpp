#include <ncurses.h>
#include <iostream>
#include <algorithm>

namespace {
constexpr int key_a = 97;
constexpr int key_s = 115;
constexpr int key_d = 100;
constexpr int key_w = 119;
}// namespace

class Snake
{
public:
  void step(int key_number);
  std::pair<int, int> getPosition();

private:
  int x = 5, y = 5;
};

void Snake::step(int key_number)
{
  if (key_number == key_a && x > 2)
    x--;
  else if (key_number == key_s && y < 18)
    y++;
  else if (key_number == key_d && x < 37)
    x++;
  else if (key_number == key_w && y > 1)
    y--;

  move(y, x);
  printw("&");
}

std::pair<int, int> Snake::getPosition()
{
  return { x, y };
}

void printBoard()
{
  char line[41];
  char borders[41];

  borders[0] = '|';
  borders[1] = '|';
  std::fill(line, line + 40, '=');
  borders[38] = '|';
  borders[39] = '|';

  line[40] = '\0';
  borders[40] = '\0';

  std::fill(borders + 2, borders + 40 - 2, ' ');
  printw(line);
  int i = 0;
  for (i = 1; i < 19; i++) {
    move(i, 0);
    printw(borders);
  }

  move(i, 0);
  printw(line);
}

int main(int, const char **)
{
  initscr();
  printBoard();
  Snake snake;
  int key_number;
  while ((key_number = getch()) != 27) {
    clear();
    printBoard();
    snake.step(key_number);
    move(20, 0);
    auto [x, y] = snake.getPosition();
    printw("x = %d, y = %d\n", x, y);
  }
  endwin();

  return 0;
}
