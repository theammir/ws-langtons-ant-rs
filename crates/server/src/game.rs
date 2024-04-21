const DIRECTIONS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct Game {
    pub w: u8,
    pub h: u8,

    pub state: Vec<Vec<u8>>,
    pub ant: (u8, u8),
    dir_index: usize,
}

impl Game {
    pub fn new(w: u8, h: u8) -> Self {
        Self {
            w,
            h,
            state: vec![vec![0; w as usize]; h as usize],
            ant: ((w / 2) as u8, u8::max((h / 2) as u8 - 2, 0)),
            dir_index: 0,
        }
    }

    pub fn tick(&mut self) {
        // Check if the ant is out of bounds
        if self.ant.0 >= self.w || self.ant.1 >= self.h {
            return;
        }

        // Toggle the ant's cell
        self.state[self.ant.1 as usize][self.ant.0 as usize] ^= 1;

        // Turn the ant
        match self.state[self.ant.1 as usize][self.ant.0 as usize] {
            0 => {
                // Turn left
                self.dir_index = (self.dir_index + 3) % 4;
            }
            1 => {
                // Turn right
                self.dir_index = (self.dir_index + 1) % 4;
            }
            _ => unreachable!(),
        }

        // Move the ant (casting is ugly af)
        self.ant.0 = (self.ant.0 as i16 + DIRECTIONS[self.dir_index].0 as i16) as u8;
        self.ant.1 = (self.ant.1 as i16 + DIRECTIONS[self.dir_index].1 as i16) as u8;
    }
}
