extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use std::collections::LinkedList;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


const UNIT: u32 = 20;
const WIDTH: u32 = UNIT * 30;
const HEIGHT: u32 = UNIT * 20;
const THICKNESS: u32 = UNIT;


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

    food: Point,

    can_turn: bool,
    inited: bool,
    //wall: [Point; 4]
}


impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let body = &mut self.body;

        self.gl.draw(args.viewport(),  |c, gl| {


            body.iter().for_each(|p: &Point| {
//                println!("{:?}", p);
                let body_square = rectangle::square(p.x as f64, p.y as f64, UNIT as f64);
                if p == body.front().unwrap(){
                    rectangle(color::BLACK, body_square, c.transform, gl);
                }
                else {
                    rectangle(color::hex("FFFF00"), body_square, c.transform, gl);
                }
            });
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.state == State::Started {
            self.inited = false;

            let body = &mut self.body;
            let dir = &self.dir;
            let mut last = Point{x:0, y:0};

            if body.len() == 1 {
                last = body.pop_back().unwrap();
            }
            else if body.len() > 1{
                body.pop_back();
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

    fn hit(&mut self) -> bool{
//        self.  碰撞检测！！
        let head = self.body.front().unwrap();
        if head.x < THICKNESS || head.x > WIDTH - (THICKNESS+UNIT) || head.y < THICKNESS || head.y > HEIGHT - (THICKNESS+UNIT){
            true
        }
        else {
            false
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
        self.can_turn = true;
        self.inited = true;
    }


}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_3;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Snake Game",
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
        food: Point{x: 0, y: 0},

        can_turn: true,
        inited: true,
    };



    game.init();

    let mut events = Events::new(EventSettings::new()).ups(2u64);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
//            println!("{:?}", game.can_turn);
            game.draw_wall(&r);
            game.render(&r);
            if game.hit(){
                game.init();
            }

            if game.state == State::Stopped && game.inited == false{
                game.init();
            }
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
            game.can_turn = true;
        }

        if let Some(Button::Keyboard(key)) = e.press_args(){
            game.button_pressed(key)
        }
    }
}