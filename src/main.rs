use macroquad::prelude::*;

struct Pipe {
        top: Rect,
        bottom: Rect,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // VARIABLES
    // Circle(x,y,r)
    let mut birb = Circle::new(60., screen_height()/2., 25.);
    let gravity = 0.2;
    let mut birb_velocity = 0.;
    let jump_velocity = -6.;
    
    let mut pipe_x = screen_width()-50.;
    let mut pipe = Pipe {
        top: Rect::new(pipe_x, 0., 50., screen_height()/2.-100.),
        bottom: Rect::new(pipe_x, screen_height()/2.+100., 50., screen_height())
    };
    let pipe_speed = -8.;
    
    let mut score:u32  = 0;
    loop {
        // draw backdround
        clear_background(SKYBLUE);

        // draw birb (x, y, radius, color)
        draw_circle(birb.x, birb.y, birb.r, YELLOW);
        // draw pipe
        draw_rectangle(pipe.top.x,pipe.top.y,pipe.top.w,pipe.top.h, GREEN);
        draw_rectangle(pipe.bottom.x,pipe.bottom.y,pipe.bottom.w,pipe.bottom.h, GREEN);

        // draw score
        draw_text(score.to_string().as_str(), 60., 60.,50., GRAY);

        // GRAVITY
        birb_velocity += gravity;
        birb.y += birb_velocity;
        
        // PIPE MOVEMENT;
        pipe_x += pipe_speed;
        // JUMP
        if is_key_pressed(KeyCode::Space) {
            birb_velocity = jump_velocity;
        }

        pipe.top.x = pipe_x;
        pipe.bottom.x = pipe_x;

        // pipe reset
        if pipe_x <= -50. {
            pipe_x = screen_width();
            let limit = 100.;
            pipe.top.h = rand::gen_range(limit, screen_height()-limit);
            pipe.bottom.y = pipe.top.h + 200.;
            score += 1;
        }        

        // Game over
        if birb.y+birb.r >= screen_height() {
            birb.y = screen_height()/2.;    
            birb_velocity = 0.; 
            pipe_x = screen_width();
            let limit = 100.;
            pipe.top.h = rand::gen_range(limit, screen_height()-limit);
            pipe.bottom.y = pipe.top.h + 200.;
            score = 0;
        }
        if birb.overlaps_rect(&pipe.top) || birb.overlaps_rect(&pipe.bottom) {
            birb.y = screen_height()/2.;    
            birb_velocity = 0.; 
            pipe_x = screen_width();
            let limit = 100.;
            pipe.top.h = rand::gen_range(limit, screen_height()-limit);
            pipe.bottom.y = pipe.top.h + 200.;
            score = 0;
        }
        
        next_frame().await
    }
}
