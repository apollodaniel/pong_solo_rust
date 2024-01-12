use std::{time::Instant, process::exit};

use piston::{ButtonEvent, Key, EventLoop};
use piston_window::{clear, rectangle, ellipse, text, CharacterCache, glyph_cache::{self, rusttype::GlyphCache}, Transformed, types::FontSize, TextureSettings, Glyphs};
use rand::Rng;

fn main() {

    // main runtime function

    const WINDOW_WIDTH: f64 = 1280.0;   
    const WINDOW_HEIGHT: f64 = 720.0;   


    
    let mut window: piston_window::PistonWindow = piston::WindowSettings::new("Pong solo!", [WINDOW_WIDTH,WINDOW_HEIGHT])
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .expect("Error building piston window");
    window.set_max_fps(60);

    const PLAYER_HEIGHT: f64 = 250.0;
    const PLAYER_WIDTH: f64 = 30.0;
    const MAX_PLAYER_DISTANCE: f64 = WINDOW_WIDTH * 0.3;
    const TILE_COUNT: f64 = 10.0;
    const TILE_SIZE: f64 = WINDOW_HEIGHT / (TILE_COUNT * 2.0);

    let mut player_y: f64 = (WINDOW_HEIGHT / 2.0) - (PLAYER_HEIGHT / 2.0); 
    let mut player_x: f64 = 100.0; 
    let mut player_direction = (false,false,false,false);
    let player_speed = 400.0;
    
    const BALL_SIZE: f64 = 50.0;
    let mut ball_x = (WINDOW_WIDTH / 2.0)  - (BALL_SIZE / 2.0);
    let mut ball_y = (WINDOW_HEIGHT / 2.0)  - (BALL_SIZE / 2.0);

    let mut ball_y_direction = if rand::thread_rng().gen_bool(0.5){1.0}else{-1.0};
    let mut ball_x_direction = if rand::thread_rng().gen_bool(0.5){1.0}else{-1.0};
    let ball_speed = 400.0;

    let mut last_time = Instant::now();
    const FONT_SIZE: u32 = 32;
    
    let font = "./minecraft.ttf";

    let mut score: u16 = 0;
    
    let mut score_text = format!("Score: {}", score);
    //let mut glyphs: GlyphCache<'_, (), _> = GlyphCache::new(font, (), TextureSettings::new()).unwrap();
    
    //let factory = window.factory.clone()
    let mut glyph= window.load_font(font).unwrap();
    while let Some(event) = window.next(){
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f64();
        last_time = current_time;
        
        window.draw_2d(&event, |c,g,d|{
            clear([0.0,0.0,0.0,1.0], g);
            let text_width = glyph.width(FONT_SIZE, &score_text).unwrap();
            let transform = c.transform.trans_pos(((WINDOW_WIDTH/2.0) - text_width as f64 / 2.0, (FONT_SIZE+50)as f64));
            ..=text::Text::new_color([1.0, 1.0, 1.0, 1.0], FONT_SIZE).draw(
                score_text.as_str(),
                &mut glyph,
                &c.draw_state,
                transform, g
            );
            rectangle([1.0,1.0,1.0,1.0], [player_x,player_y, PLAYER_WIDTH, PLAYER_HEIGHT], c.transform, g);
            
            ellipse([1.0,1.0,1.0,1.0], [ball_x,ball_y, BALL_SIZE, BALL_SIZE], c.transform, g);
            
            

            for i in 0..TILE_COUNT as usize{
                rectangle([1.0,1.0,1.0,1.0], [MAX_PLAYER_DISTANCE+30.0, TILE_SIZE*(i*2) as f64, 100.0 / TILE_COUNT, TILE_SIZE], c.transform, g);
            }
            glyph.factory.encoder.flush(d);
        });

        event.button(|f|{
            if let piston_window::input::Button::Keyboard(key) = f.button {
                match f.state {
                    piston::ButtonState::Press => {
                        match key {
                            Key::S => player_direction.0 = true,
                            Key::W => player_direction.1 = true,
                            Key::A => player_direction.2 = true,
                            Key::D => player_direction.3 = true,
                            _ =>{}
                        }
                    }
                    piston::ButtonState::Release=> {
                        match key {
                            Key::S => player_direction.0 = false,
                            Key::W => player_direction.1 = false,
                            Key::A => player_direction.2 = false,
                            Key::D => player_direction.3 = false,
                            _ =>{}
                        }
                    }
                }
            }
        });

        move_player(&mut player_y,
            &mut player_x,
            delta_time,
            &player_speed,
            player_direction
        );
        move_ball(&mut ball_x, &mut ball_y, ball_speed, &mut ball_x_direction, &mut ball_y_direction, delta_time,player_x, player_y,&mut score,&mut score_text);
    }

    fn move_ball(ball_x: &mut f64, ball_y: &mut f64, ball_speed: f64, ball_x_direction: &mut f64,ball_y_direction: &mut f64, delta_time: f64, 
        player_x: f64, player_y: f64,score: &mut u16, score_text: &mut String
    ){
        // ball should invert value when collides with right wall, top, and bottom, and with the player
        // when interacts with the left wall the game might close
        let mut ball_x_movement = ball_speed * *ball_x_direction *  delta_time;
        let mut ball_y_movement = ball_speed * *ball_y_direction *delta_time;

        // horizontal collision
        if (*ball_x + ball_x_movement) >= (WINDOW_WIDTH - BALL_SIZE)
        {
            // when touches right 
            ball_x_movement *= -1.0;
            *ball_x_direction *= -1.0;
        }

        if  ((*ball_x + ball_x_movement) <= (PLAYER_WIDTH + player_x) && (*ball_x + ball_x_movement) >= player_x) 
        && (*ball_y + BALL_SIZE) > player_y && (*ball_y + BALL_SIZE) < (player_y + PLAYER_HEIGHT) {
            ball_x_movement *= -1.0;
            *ball_x_direction *= -1.0;
            *score += 1;
            *score_text = format!("Score: {}", score);
        }

        if (*ball_x + ball_x_movement) <= 0.0{
            // touches left
            exit(0);
        }
        
        // vertical collision
        if (*ball_y + ball_y_movement) >= (WINDOW_HEIGHT - BALL_SIZE) || (*ball_y + ball_y_movement) <= 0.0 {
            // when touches bottom or top
            ball_y_movement *= -1.0;
            *ball_y_direction *= -1.0;
        }

        *ball_x += ball_x_movement;
        *ball_y += ball_y_movement;
    }

    fn move_player(player_y: &mut f64, player_x: &mut f64,
            delta_time: f64, player_speed: &f64,
            player_vertical_direction: (bool, bool,bool,bool)
        ){
        let player_y_movement = if player_vertical_direction.0 {player_speed * delta_time} else if player_vertical_direction.1 {-(player_speed * delta_time)} else {0.0};
        let player_x_movement = if player_vertical_direction.3 {player_speed * delta_time} else if player_vertical_direction.2 {-(player_speed * delta_time)} else {0.0};

        // collisions 
        if (*player_y + player_y_movement) > 0.0 && (*player_y + player_y_movement) < (WINDOW_HEIGHT - PLAYER_HEIGHT){
            *player_y += player_y_movement; 
        }

        if (*player_x + player_x_movement) > 0.0 && (*player_x + player_x_movement) < (MAX_PLAYER_DISTANCE - PLAYER_WIDTH){
            *player_x += player_x_movement; 
        }
    }

}
