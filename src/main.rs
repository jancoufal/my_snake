#[derive(Debug, Copy, Clone)]
struct Point2D { x: i32, y: i32, }

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

#[derive(Debug, Copy, Clone)]
enum CellType { Border, Empty, SnakeBody, Food, }

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
}

impl Game {
    fn new(cols: usize, rows: usize) -> Game {
        let mut field = vec![vec![Cell { cell_type: CellType::Empty }; cols]; rows];

        // make border
        for (row_index, row) in &field.iter().enumerate()
        {
            for (col_index, cell) in &row.iter().enumerate()
            {
                if row_index == 0 || row_index == rows - 1 || col_index == 0 || col_index == cols - 1
                {
                    cell.cell_type = CellType::Border;
                }
            }
        }

        Game { state: GameState::Paused, field }
    }
}

fn main() {
    let (cols, rows) = (20, 20);

    let mut game = Game::new(cols, rows);



    println!("Hello, world!");
}
