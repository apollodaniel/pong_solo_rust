use std::time::Instant;

use piston::{ButtonEvent, Key, EventLoop};
use piston_window::{clear, rectangle};

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
    let mut player_vertical_direction = (false,false);
    let player_speed = 600.0;
    

    let mut last_time = Instant::now();
    while let Some(event) = window.next(){
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f64();
        last_time = current_time;

        window.draw_2d(&event, |c,g,_|{
            clear([0.0,0.0,0.0,1.0], g);
            rectangle([1.0,1.0,1.0,1.0], [100.0,player_y, PLAYER_WIDTH, PLAYER_HEIGHT], c.transform, g);
            
            
            
            for i in 0..TILE_COUNT as usize{
                rectangle([1.0,1.0,1.0,1.0], [MAX_PLAYER_DISTANCE+30.0, TILE_SIZE*(i*2) as f64, 100.0 / TILE_COUNT, TILE_SIZE], c.transform, g);
            }
        });

        event.button(|f|{
            if let piston_window::input::Button::Keyboard(key) = f.button {
                match f.state {
                    piston::ButtonState::Press => {
                        match key {
                            Key::W => player_vertical_direction.1 = true,
                            Key::S => player_vertical_direction.0 = true,
                            _ =>{}
                        }
                    }
                    piston::ButtonState::Release=> {
                        match key {
                            Key::W => player_vertical_direction.1 = false,
                            Key::S => player_vertical_direction.0 = false,
                            _ =>{}
                        }
                    }
                }
            }
        });
        move_player_y(&mut player_y,
            delta_time,
            &player_speed,
            player_vertical_direction
        );
    }

    fn move_player_y(player_y: &mut f64, delta_time: f64, player_speed: &f64, player_vertical_direction: (bool, bool)){
        let player_y_movement = if player_vertical_direction.0 {player_speed * delta_time} else if player_vertical_direction.1 {-(player_speed * delta_time)} else {0.0};
        
        // collisions 
        if (*player_y + player_y_movement) > 0.0 && (*player_y + player_y_movement) < (WINDOW_HEIGHT - PLAYER_HEIGHT){
            *player_y += player_y_movement; 
        }
    }

}
