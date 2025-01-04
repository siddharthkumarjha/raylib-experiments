#include "raylib.h"

struct pos
{
    int32_t x;
    int32_t y;
    int32_t dx;
    int32_t dy;
};

void boundsCheck(pos &cur_pos)
{
    cur_pos.x += cur_pos.dx;
    cur_pos.y += cur_pos.dy;

    if (cur_pos.x == 0 || cur_pos.x == (GetScreenWidth() - 50))
    {
        cur_pos.dx = -cur_pos.dx;
    }
    if (cur_pos.y == 0 || cur_pos.y == (GetScreenHeight() - 50))
    {
        cur_pos.dy = -cur_pos.dy;
    }
}

int main(void)
{

    auto cur_pos = pos {
        .x= 0, .y= 0, .dx= 10, .dy= 10
    };

    InitWindow(500, 500, "My first window");
    SetTargetFPS(60);

    while(!WindowShouldClose())
    {
        BeginDrawing();
        ClearBackground(GetColor(0x181818ff));
        DrawRectangle(cur_pos.x, cur_pos.y, 50, 50, RED);
        EndDrawing();

        boundsCheck(cur_pos);
    }
    CloseWindow();
    return 0;
}
