enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Moving on up"),
        Movement::Down => println!("Moving on down"),
        Movement::Left => println!("Moving on left"),
        Movement::Right => println!("Moving on right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
}
