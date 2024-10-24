#[derive(Debug, Copy, Clone)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

#[derive(Debug, Copy, Clone)]
enum CellType {
    Empty,
    Border,
    SnakeHead,
    SnakeBody,
    SnakeTail,
    Food,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    cell_type: CellType,
}

#[derive(Debug, Copy, Clone)]
enum GameOverType {
    PlaygroundFilled,
    BorderHit,
    SelfBite
}

#[derive(Debug, Copy, Clone)]
enum GameState {
    Paused,
    Playing,
    GameOver(GameOverType),
}

#[derive(Debug)]
struct Game {
    state: GameState,
    field: Vec<Vec<Cell>>,
    direction: Direction,
    snake_length: usize,
    step_count: usize,
}

impl Game {
    fn new(cols: usize, rows: usize) -> Game {
        let mut field = vec![vec![Cell { cell_type: CellType::Empty }; cols]; rows];

        // make border
        for (y, row) in field.iter_mut().enumerate()
        {
            for (x, cell) in row.iter_mut().enumerate()
            {
                if y == 0 || y == rows - 1 || x == 0 || x == cols - 1
                {
                    cell.cell_type = CellType::Border;
                }
            }
        }

        let (mid_x, mid_y) = (cols / 2, rows / 2);
        field[mid_y][mid_x].cell_type = CellType::SnakeHead;

        Game {
            state: GameState::Paused,
            field,
            direction: Direction::None,
            snake_length: 1usize,
            step_count: 0usize,
        }
    }

    fn play(&mut self, direction: Direction) {
        self.state = GameState::Playing;
        self.direction = direction;
    }
}

fn main() {
    let (cols, rows) = (10, 10);

    let mut game = Game::new(cols, rows);

    for col in game.field {
        for cell in col {
            print!("{}", match cell.cell_type {
                CellType::Empty => " ".to_string(),
                CellType::Border => "#".to_string(),
                CellType::SnakeHead => "@".to_string(),
                CellType::SnakeBody => "*".to_string(),
                CellType::SnakeTail => ".".to_string(),
                CellType::Food => "Q".to_string(),
            });
        }
        println!();
    }
}
