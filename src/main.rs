use rand::seq::SliceRandom;

use wave_fn_collapse::Tile;
use wave_fn_collapse::TileKind;

fn main() {
    let size = 20;
    let width = 2 * size;
    let height = size;
    let initial = Tile {
        index: 0,
        collapsed: false,
        options: vec![
            TileKind::Blank,
            TileKind::Up,
            TileKind::Right,
            TileKind::Down,
            TileKind::Left,
            TileKind::UpRight,
            TileKind::UpLeft,
            TileKind::DownRight,
            TileKind::DownLeft,
            TileKind::Vertical,
            TileKind::Horizontal,
            TileKind::UpEnd,
            TileKind::RightEnd,
            TileKind::DownEnd,
            TileKind::LeftEnd,
            TileKind::Cross,
        ],
    };
    let mut grid: Vec<Tile> = Vec::new();
    for i in 0..width * height {
        let mut tile = initial.clone();
        tile.index = i;
        grid.push(tile);
    }
    loop {
        let mut grid_sorted = grid.clone();
        grid_sorted = grid_sorted.into_iter().filter(|x| !x.collapsed).collect();
        grid_sorted.sort_by(|a, b| a.options.len().cmp(&b.options.len()));
        let min = grid_sorted[0].options.len();
        grid_sorted = grid_sorted
            .into_iter()
            .filter(|x| min == x.options.len())
            .collect();
        let mut tile = grid_sorted
            .choose(&mut rand::thread_rng())
            .expect("cannot pick random tile!")
            .clone();
        let index = tile.index;
        let y = index / width;
        let x = index % width;
        let mut sides: [Option<usize>; 4] = [None; 4];
        if x > 0 {
            let left_tile = &grid[index - 1];
            if left_tile.collapsed {
                let kind = &left_tile.options[0];
                sides[3] = Some(kind.sides()[1]);
            }
        } else {
            sides[3] = Some(0);
        }
        if x < width - 1 {
            let right_tile = &grid[index + 1];
            if right_tile.collapsed {
                let kind = &right_tile.options[0];
                sides[1] = Some(kind.sides()[3]);
            }
        } else {
            sides[1] = Some(0);
        }
        if y > 0 {
            let up_tile = &grid[index - width];
            if up_tile.collapsed {
                let kind = &up_tile.options[0];
                sides[0] = Some(kind.sides()[2]);
            }
        } else {
            sides[0] = Some(0);
        }
        if y < height - 1 {
            let down_tile = &grid[index + width];
            if down_tile.collapsed {
                let kind = &down_tile.options[0];
                sides[2] = Some(kind.sides()[0]);
            }
        } else {
            sides[2] = Some(0);
        }
        let mut options_filtered = tile.options;
        options_filtered = options_filtered
            .into_iter()
            .filter(|x| x.fit(&sides))
            .collect();
        let option = options_filtered
            .choose(&mut rand::thread_rng())
            .expect("cannot pick kind!")
            .clone();
        tile.collapsed = true;
        tile.options = vec![option];
        grid[index] = tile;
        if grid.iter().all(|x| x.collapsed) {
            break;
        }
    }
    draw(&grid, width, height);
}

#[allow(dead_code)]
fn draw(grid: &Vec<Tile>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            if grid[i].options.len() > 1 {
                print!("·");
                continue;
            }
            print!("{}", grid[i].options[0]);
        }
        println!("");
    }
}
