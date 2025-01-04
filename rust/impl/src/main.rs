use raylib as rl;

struct Pos
{
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

fn bounds_check(cur_pos: &mut Pos)
{
    cur_pos.x += cur_pos.dx;
    cur_pos.y += cur_pos.dy;

    if cur_pos.x == 0 || cur_pos.x == (rl::get_screen_width() - 50)
    {
        cur_pos.dx = -cur_pos.dx;
    }
    if cur_pos.y == 0 || cur_pos.y == (rl::get_screen_height() - 50)
    {
        cur_pos.dy = -cur_pos.dy;
    }
}

fn main() 
{
    let mut cur_pos: Pos = Pos {
        x: 0, y: 0, dx: 10, dy: 10
    };
    rl::init_window(500, 500, "My first window");
    rl::set_target_fps(60);
    while !rl::window_should_close()
    {
        rl::begin_drawing();
        rl::clear_bg(rl::get_color(0x181818ff));
        rl::draw_rectangle(cur_pos.x.clone(), cur_pos.y.clone(), 50, 50, rl::RED);
        rl::end_drawing();

        bounds_check(&mut cur_pos);
    }
}
