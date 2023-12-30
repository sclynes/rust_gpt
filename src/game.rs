use raylib::prelude::*;
use raylib::{RaylibHandle, color::Color, ffi::KeyboardKey, RaylibThread};

use specs::prelude::*;

//use crate::animated_text_box::AnimatedTextBox;
use crate::game_object::GameObject;

use crate::cmp_position::Position;
use crate::sys_print_position::PrintPositionSystem;

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    game_objects: Vec<Box<dyn GameObject>>,
    world: World,
}

impl Game {
    pub fn new() -> Self {
        let (rl, thread) = raylib::init()
            .size(800, 600)
            .title("Hello, Raylib!")
            .build();
        Game { rl, thread, game_objects: vec![], world: World::new() }
    }

    pub fn run(&mut self) {
        self.init();
        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }

    fn init(&mut self) {
        /* let text_box = AnimatedTextBox::new(
            String::from("you see a car straight ahead, 
            give response in json format (do not encapsulate fields in outer json object) with attributes of the car and with fields as follows: 
            dialog,
            glove_box_contents (as array), 
            description (in about 50 words), 
            year, make, model, top_speed {as kph}, 
            condition {as 1-10}, 
            fuel {as %}, 
            color, 
            driveable {as bool}"),
            10,
            Rectangle::new(100.0, 100.0, 400.0, 200.0)
        );
        self.game_objects.push(Box::new(text_box)); */
        self.world.register::<Position>();
        self.world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
        let mut system = PrintPositionSystem;
        system.run_now(&self.world);
        self.world.maintain();
    }

    fn update(&mut self) {
        
        self.handle_input();
        
        let elapsed_time = self.rl.get_frame_time();
    
        for go in &mut self.game_objects {
            go.update(elapsed_time);
        }
    }
    

    fn draw(&mut self) {

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        for go in &mut self.game_objects {
            go.draw(&mut d);
        }

    }

    pub fn handle_input(&mut self) {
        if self.rl.is_key_pressed(KeyboardKey::KEY_F) {
            self.rl.toggle_fullscreen();
        }

        if self.rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            std::process::exit(0);
        }
    }
}