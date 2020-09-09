#![allow(unused)]
use std::collections::HashMap;
use gilrs::{Gilrs, Gamepad, Button, Event, Axis};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Stick {
    Left(Direction),
    Right(Direction),
}

#[derive(Debug)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct Combo {
    vec: Vec<Stick>,
    map: HashMap<Vec<Stick>, Letter>,
}

impl Combo {
    fn check_bounds_and_push(&mut self, gamepad: Gamepad, x: Axis, y: Axis, north: Stick, east: Stick, south: Stick, west: Stick) {
        let min = 0.9;
        let mut stick = None;

        match (gamepad.value(x), gamepad.value(y)) {
          (x, y) => {
              if x.abs() > y.abs() {
                  if x >= min {
                      stick = Some(east);
                  } else if x <= -min {
                      stick = Some(west);
                  }
              } else {
                  if y >= min {
                      stick = Some(north);
                  } else if y <= -min {
                      stick = Some(south);
                  }
              }
          },
          _ => {}
        }

        match (&stick, self.vec.last()) {
            (Some(a), Some(b)) if a != b => self.vec.push(*a),
            (Some(a), None) => self.vec.push(*a),
            _ => (),
        }

        /*
        match &stick {
            Some(_a) => {
                if self.vec.len() > 0 {
                    println!("vec is {:?}", self.vec);
                }
            },
            _ => (),
        }
        */
    }

    fn check_zero(&mut self, gamepad: Gamepad) {
        let min = 0.5;

        if gamepad.value(Axis::RightStickX) < min && gamepad.value(Axis::RightStickX) > -min
            && gamepad.value(Axis::RightStickY) < min && gamepad.value(Axis::RightStickY) > -min
            && gamepad.value(Axis::LeftStickX) < min && gamepad.value(Axis::LeftStickX) > -min
            && gamepad.value(Axis::LeftStickY) < min && gamepad.value(Axis::LeftStickY) > -min {
                if self.vec.len() > 0 {
                    match self.map.get(&self.vec) {
                        Some(a) => println!("{:?}", *a),
                        _ => (),
                    }

                    self.vec.clear();
                }
        }
    }

    fn process(&mut self, gamepad: Gamepad) {
        if gamepad.is_pressed(Button::South) {
            println!("GAMEPAD: {:#?}", gamepad);
        }

        self.check_bounds_and_push(gamepad, Axis::LeftStickX, Axis::LeftStickY,
                                   Stick::Left(Direction::North), 
                                   Stick::Left(Direction::East), 
                                   Stick::Left(Direction::South),
                                   Stick::Left(Direction::West));
        self.check_bounds_and_push(gamepad, Axis::RightStickX, Axis::RightStickY,
                                   Stick::Right(Direction::North), 
                                   Stick::Right(Direction::East), 
                                   Stick::Right(Direction::South),
                                   Stick::Right(Direction::West));

        self.check_zero(gamepad);
    }

    fn setup(&mut self) {
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::West)],
            Letter::A
        );
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::West), Stick::Left(Direction::South)],
            Letter::B
        );
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::West), Stick::Left(Direction::South), Stick::Left(Direction::East)],
            Letter::C
        );
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::West), Stick::Left(Direction::South), Stick::Left(Direction::East), Stick::Left(Direction::North)],
            Letter::D
        );

        self.map.insert(
            vec![Stick::Left(Direction::West), Stick::Left(Direction::South)],
            Letter::E
        );
        self.map.insert(
            vec![Stick::Left(Direction::West), Stick::Left(Direction::South), Stick::Left(Direction::East)],
            Letter::F
        );
        self.map.insert(
            vec![Stick::Left(Direction::West), Stick::Left(Direction::South), Stick::Left(Direction::East), Stick::Left(Direction::North)],
            Letter::G
        );
        self.map.insert(
            vec![Stick::Left(Direction::West), Stick::Left(Direction::South), Stick::Left(Direction::East), Stick::Left(Direction::North), Stick::Left(Direction::West)],
            Letter::H
        );

        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::East)],
            Letter::I
        );
        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::East), Stick::Left(Direction::North)],
            Letter::J
        );
        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::East), Stick::Left(Direction::North), Stick::Left(Direction::West)],
            Letter::K
        );
        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::East), Stick::Left(Direction::North), Stick::Left(Direction::West), Stick::Left(Direction::South)],
            Letter::L
        );

        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::North)],
            Letter::M
        );
        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::North), Stick::Left(Direction::West)],
            Letter::N
        );
        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::North), Stick::Left(Direction::West), Stick::Left(Direction::South)],
            Letter::O
        );
        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::North), Stick::Left(Direction::West), Stick::Left(Direction::South), Stick::Left(Direction::East)],
            Letter::P
        );

        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::East)],
            Letter::Q
        );
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::East), Stick::Left(Direction::South)],
            Letter::R
        );
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::East), Stick::Left(Direction::South), Stick::Left(Direction::West)],
            Letter::S
        );
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::East), Stick::Left(Direction::South), Stick::Left(Direction::West), Stick::Left(Direction::North)],
            Letter::T
        );

        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::South)],
            Letter::U
        );
        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::South), Stick::Left(Direction::West)],
            Letter::V
        );
        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::South), Stick::Left(Direction::West), Stick::Left(Direction::North)],
            Letter::W
        );
        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::South), Stick::Left(Direction::West), Stick::Left(Direction::North), Stick::Left(Direction::East)],
            Letter::X
        );

        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::West)],
            Letter::Y
        );
        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::West), Stick::Left(Direction::North)],
            Letter::Z
        );
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad;

    let mut combo = Combo {
        vec: Vec::new(),
        map: HashMap::new(),
    };

    combo.setup();

    loop {
        while let Some(Event { id, event: _, time: _ }) = gilrs.next_event() {
            active_gamepad = Some(id);

            if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                combo.process(gamepad);
            }
        }
    }
}
