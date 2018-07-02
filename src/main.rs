extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate rand;


use std::collections::LinkedList;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use rand::prelude::*;


const UNIT: u32 = 30;                 //单位方块大小
const WIDTH: u32 = UNIT * 30;
const HEIGHT: u32 = UNIT * 20;
const THICKNESS: u32 = UNIT;          //墙的厚度，不要改

const BASESPEED: u64 = 2;             //最低速度

const TITLE: &str  = "SnakeGame Press Home to start, end to stop";


#[derive(Debug, Clone, Eq, PartialEq)]
struct Point{
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
enum State {
    Started,
    Stopped,
    Paused,
}


pub struct Game {

    gl: GlGraphics, // OpenGL drawing backend.

    dir: Direction,
    body: LinkedList<Point>,

    state: State,

    score: u32,
    goal: bool,
    food: Point,

    lv: u32,

    can_turn: bool,
    inited: bool,
    //wall: [Point; 4]
}


impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let body = &mut self.body;
        let food = &self.food;

        self.gl.draw(args.viewport(),  |c, gl| {


            body.iter().for_each(|p: &Point| {
//                println!("{:?}", p);
                let body_square = rectangle::square(p.x as f64, p.y as f64, UNIT as f64);
                if p == body.front().unwrap(){
                    // snake head pure black
                    rectangle(color::BLACK, body_square, c.transform, gl);
                }
                else {
                    // snake body light gray
                    rectangle(color::hex("555555"), body_square, c.transform, gl);
                }

                let food_square = rectangle::square(food.x as f64, food.y as f64, UNIT as f64);
                // let food to be red, why not?
                rectangle(color::hex("FF0000"), food_square, c.transform, gl);
            });
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.state == State::Started {
            self.inited = false;

            let body = &mut self.body;
            let dir = &self.dir;
            let mut last = Point{x:0, y:0};

            if body.len() == 1 {
                // not possible
                if !self.goal{
                    last = body.pop_back().unwrap();
                }
                else {
                    last = body.front().unwrap().clone();
                }
            }
            else if body.len() > 1{
                if !self.goal{
                    body.pop_back();
                }
                self.goal = false;
                last = (*body.front().unwrap()).clone();
            }

            let p = match *dir {
                Direction::Up => Point{x: last.x, y: last.y - UNIT},
                Direction::Down => Point{x: last.x, y: last.y + UNIT},
                Direction::Left => Point{x: last.x - UNIT, y: last.y},
                Direction::Right => Point{x: last.x + UNIT, y: last.y},
            };
            body.push_front(p);
        }
    }

    fn draw_wall(&mut self, args: &RenderArgs){
        use graphics::*;
        let gl: &mut GlGraphics = &mut self.gl;

        //const BLACK: [f32; 4] = [ 0f32, 0f32, 0f32, 1f32 ];
//        let l_wall = rectangle::Rectangle{color: BLACK, shape: rectangle::Shape::Square, border: BLACK};
        let wall = Rectangle::new(color::BLACK);
        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear(color::WHITE, gl);
            wall.draw([0 as f64,0 as f64,WIDTH as f64, THICKNESS as f64], &c.draw_state, c.transform, gl);
            wall.draw([0 as f64,(HEIGHT - THICKNESS) as f64,WIDTH as f64, THICKNESS as f64], &c.draw_state, c.transform, gl);
            wall.draw([0 as f64,0 as f64,THICKNESS as f64, HEIGHT as f64], &c.draw_state, c.transform, gl);
            wall.draw([(WIDTH - THICKNESS) as f64,0 as f64,THICKNESS as f64, HEIGHT as f64], &c.draw_state, c.transform, gl);
        });

    }

    fn generate_food(&mut self){
        // generate new food position
        // need random crate
        let mut rng = thread_rng();
        let body = &self.body;

        loop {
            let x = rng.gen_range(1, 29);
            let y = rng.gen_range(1, 19);
            self.food = Point{x: x * UNIT, y: y * UNIT};
            if !body.contains(&self.food){
                break;
            }
        }

    }


    fn hit(&mut self) -> bool{
//        self.  碰撞检测！！
        let head = self.body.front().unwrap();
        let body = &self.body;
        if head.x < THICKNESS || head.x > WIDTH - (THICKNESS+UNIT) || head.y < THICKNESS || head.y > HEIGHT - (THICKNESS+UNIT) {
            // hit the wall
            true
        }
        else if body.iter().filter_map(|b| {
            if b.x == head.x && b.y == head.y {
                Some(b)
            }
            else{
                None
            }
        }).collect::<LinkedList<&Point>>().len() > 1 {
//            println!("{:?}", body.iter().filter_map(|b| {
//                if b.x == head.x && b.y == head.y {
//                    Some(b)
//                }
//                    else{
//                        None
//                    }
//            }).collect::<LinkedList<&Point>>());
            // hit itself
            true
        }
        else {
            // didnot hit anything
            false
        }
    }

    fn eat(&mut self)-> bool{
        // if ate a food, just set goal to true; update func will handle it
        let head = self.body.front().unwrap();
        let food = &self.food;
        if head.x == food.x && head.y == food.y{
//            println!("ate!");
            true
        }
        else {
            false
        }
    }

    fn level_up(&mut self) {
        if self.lv < 8 {
            self.lv += 1;
        }
    }

    #[allow(non_snake_case)]
    fn button_pressed(&mut self, args: Key){
        let can_turn = &mut self.can_turn;
        match args{
            // Snake control
            Key::Up => {
                if self.dir != Direction::Down && *can_turn == true {
                    self.dir = Direction::Up;
                    *can_turn = false;
                }
            },
            Key::Down => {
                if self.dir != Direction::Up && *can_turn == true  {
                    self.dir = Direction::Down;
                    *can_turn = false
                }
            },
            Key::Left => {
                if self.dir != Direction::Right && *can_turn == true {
                    self.dir = Direction::Left;
                    *can_turn = false
                }
            },
            Key::Right => {
                if self.dir != Direction::Left  && *can_turn == true {
                    self.dir = Direction::Right;
                    *can_turn = false
                }
            },

            // Game control
            Key::Home => {
                //Start and Pause the game
                if self.state == State::Paused || self.state == State::Stopped{
                    self.state = State::Started;
                }
                else {
                    self.state = State::Paused;
                }
            },
            Key::End => {
                //Stop the game and reset game
                self.state = State::Stopped;
            },

            _ => ()
        }
        ()
    }


    fn init(&mut self) {
        // reinit the game!
        self.state = State::Stopped;
        self.body.clear();
        self.body.push_back(Point{x:UNIT*5, y:UNIT*5});
        self.body.push_back(Point{x:UNIT*4, y:UNIT*5});
        self.body.push_back(Point{x:UNIT*3, y:UNIT*5});
// test hit itself
//        self.body.push_back(Point{x:UNIT*2, y:UNIT*5});
//        self.body.push_back(Point{x:UNIT*1, y:UNIT*5});
        self.dir = Direction::Right;
        self.can_turn = true;
        self.score = 0;
        self.goal = false;
        self.generate_food();
        self.lv = 1;


        self.inited = true;
    }


}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_3;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        TITLE,
            [WIDTH, HEIGHT]
        )
        .opengl(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        dir: Direction::Right,
        body: LinkedList::<Point>::new(),

        state: State::Stopped,

        score: 0u32,
        goal: false,
        food: Point{x: 0, y: 0},

        lv: 1,

        can_turn: true,
        inited: true,
    };

    game.init();
    window.window.set_title((TITLE.to_owned() + " Current Score: " + &game.score.to_string() + " Lv: " + &game.lv.to_string()).as_str());

    let mut events = Events::new(EventSettings::new()).ups(BASESPEED);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
//            println!("{:?}", game.can_turn);
            game.draw_wall(&r);
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
            game.can_turn = true;

            if game.hit(){
                game.init();
                window.window.set_title((TITLE.to_owned() + " Current Score: " + &game.score.to_string() + " Lv: " + &game.lv.to_string()).as_str());
            }

            if game.state == State::Stopped && game.inited == false{
                game.init();
                window.window.set_title((TITLE.to_owned() + " Current Score: " + &game.score.to_string() + " Lv: " + &game.lv.to_string()).as_str());
//                println!("{:?}", (TITLE.to_owned() + " Current Score: " + &game.score.to_string()).as_str());
//                println!("inited")
            }

            // if snake does eat a food, just set goal to true, update func will handle it
            if game.eat(){
                game.goal = true;
                game.score = game.score + 1;
                game.generate_food();
                window.window.set_title((TITLE.to_owned() + " Current Score: " + &game.score.to_string() + " Lv: " + &game.lv.to_string()).as_str());
            }

            if game.score % 5 == 0 && game.score != 0 && game.goal{
                game.level_up();
                window.window.set_title((TITLE.to_owned() + " Current Score: " + &game.score.to_string() + " Lv: " + &game.lv.to_string()).as_str());
            }


            match game.lv{
                1 => events.set_ups(BASESPEED + 0),
                2 => events.set_ups(BASESPEED + 1),
                3 => events.set_ups(BASESPEED + 2),
                4 => events.set_ups(BASESPEED + 4),
                5 => events.set_ups(BASESPEED + 6),
                6 => events.set_ups(BASESPEED + 8),
                7 => events.set_ups(BASESPEED + 10),
                8 => events.set_ups(BASESPEED + 12),
                _ => ()
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args(){
            game.button_pressed(key)
        }
    }
}