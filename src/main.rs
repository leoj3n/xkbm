use gilrs::{Axis, Button, Event, Gamepad, Gilrs};
use std::collections::HashMap;

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
    Space,
}

const MIN: f32 = 0.9;
const MAX: f32 = 0.5;

#[derive(Debug)]
struct Combo {
    vec: Vec<Stick>,
    map: HashMap<Vec<Stick>, Letter>,
}

impl Combo {
    #[allow(clippy::too_many_arguments)]
    fn check_bounds_and_push(
        &mut self,
        x: f32,
        y: f32,
        north: Stick,
        east: Stick,
        south: Stick,
        west: Stick,
    ) {
        let mut stick = None;

        if x.abs() > y.abs() {
            if x >= MIN {
                stick = Some(east);
            } else if x <= -MIN {
                stick = Some(west);
            }
        } else if y >= MIN {
            stick = Some(north);
        } else if y <= -MIN {
            stick = Some(south);
        }

        // Push if not a duplicate of previous push.
        // Q: Is there a more terse / DRY way of doing this?
        match (&stick, self.vec.last()) {
            (Some(a), Some(b)) if a != b => self.vec.push(*a),
            (Some(a), None) => self.vec.push(*a),
            _ => (),
        }
    }

    fn check_zero(&mut self, gamepad: Gamepad) {
        if self.vec.len() > 1 // This allows combos across zero space (f.ex: Letter::Space).
            && gamepad.value(Axis::RightStickX) < MAX
            && gamepad.value(Axis::RightStickX) > -MAX
            && gamepad.value(Axis::RightStickY) < MAX
            && gamepad.value(Axis::RightStickY) > -MAX
            && gamepad.value(Axis::LeftStickX) < MAX
            && gamepad.value(Axis::LeftStickX) > -MAX
            && gamepad.value(Axis::LeftStickY) < MAX
            && gamepad.value(Axis::LeftStickY) > -MAX
        {
            if let Some(a) = self.map.get(&self.vec) {
                println!("{:?}", *a)
            }
            self.vec.clear();
        }
    }

    fn process(&mut self, gamepad: Gamepad) {
        // South button prints gamepad object.
        if gamepad.is_pressed(Button::South) {
            println!("GAMEPAD: {:#?}", gamepad);
        }

        // Track left stick direction.
        self.check_bounds_and_push(
            gamepad.value(Axis::LeftStickX),
            gamepad.value(Axis::LeftStickY),
            Stick::Left(Direction::North),
            Stick::Left(Direction::East),
            Stick::Left(Direction::South),
            Stick::Left(Direction::West),
        );

        // Track right stick direction.
        self.check_bounds_and_push(
            gamepad.value(Axis::RightStickX),
            gamepad.value(Axis::RightStickY),
            Stick::Right(Direction::North),
            Stick::Right(Direction::East),
            Stick::Right(Direction::South),
            Stick::Right(Direction::West),
        );

        // Print any combo match and clear vec.
        self.check_zero(gamepad);
    }

    fn setup(&mut self) {
        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::West)],
            Letter::A,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
            ],
            Letter::B,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
            ],
            Letter::C,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
            ],
            Letter::D,
        );

        self.map.insert(
            vec![Stick::Left(Direction::West), Stick::Left(Direction::South)],
            Letter::E,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
            ],
            Letter::F,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
            ],
            Letter::G,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
            ],
            Letter::H,
        );

        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::East)],
            Letter::I,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
            ],
            Letter::J,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
            ],
            Letter::K,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
            ],
            Letter::L,
        );

        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::North)],
            Letter::M,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
            ],
            Letter::N,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
            ],
            Letter::O,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::East),
                Stick::Left(Direction::North),
                Stick::Left(Direction::West),
                Stick::Left(Direction::South),
                Stick::Left(Direction::East),
            ],
            Letter::P,
        );

        self.map.insert(
            vec![Stick::Left(Direction::North), Stick::Left(Direction::East)],
            Letter::Q,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::North),
                Stick::Left(Direction::East),
                Stick::Left(Direction::South),
            ],
            Letter::R,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::North),
                Stick::Left(Direction::East),
                Stick::Left(Direction::South),
                Stick::Left(Direction::West),
            ],
            Letter::S,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::North),
                Stick::Left(Direction::East),
                Stick::Left(Direction::South),
                Stick::Left(Direction::West),
                Stick::Left(Direction::North),
            ],
            Letter::T,
        );

        self.map.insert(
            vec![Stick::Left(Direction::East), Stick::Left(Direction::South)],
            Letter::U,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::East),
                Stick::Left(Direction::South),
                Stick::Left(Direction::West),
            ],
            Letter::V,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::East),
                Stick::Left(Direction::South),
                Stick::Left(Direction::West),
                Stick::Left(Direction::North),
            ],
            Letter::W,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::East),
                Stick::Left(Direction::South),
                Stick::Left(Direction::West),
                Stick::Left(Direction::North),
                Stick::Left(Direction::East),
            ],
            Letter::X,
        );

        self.map.insert(
            vec![Stick::Left(Direction::South), Stick::Left(Direction::West)],
            Letter::Y,
        );
        self.map.insert(
            vec![
                Stick::Left(Direction::South),
                Stick::Left(Direction::West),
                Stick::Left(Direction::North),
            ],
            Letter::Z,
        );

        self.map.insert(
            vec![Stick::Left(Direction::West), Stick::Left(Direction::East)],
            Letter::Space,
        );
    }
}

#[allow(dead_code)]
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
        while let Some(Event { id, .. }) = gilrs.next_event() {
            active_gamepad = Some(id);

            // Q: Can we get gamepad without using map?
            if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                combo.process(gamepad);
            }
        }
    }
}
