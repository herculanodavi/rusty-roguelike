use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn row_first_idx(x: u32, y: u32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn try_idx(point: Point) -> Option<usize> {
    if is_in_bounds(point) {
        Some(row_first_idx(
            point.x.unsigned_abs(),
            point.y.unsigned_abs(),
        ))
    } else {
        None
    }
}

pub fn is_in_bounds(point: Point) -> bool {
    point.x >= 0
        && point.x.unsigned_abs() < SCREEN_WIDTH
        && point.y >= 0
        && point.y.unsigned_abs() < SCREEN_HEIGHT
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /*
    pub fn tile_at(&self, point: Point) -> Option<&TileType> {
        try_idx(point).and_then(|idx| self.tiles.get(idx))
    }
    */

    pub fn mut_tile_at(&mut self, point: Point) -> Option<&mut TileType> {
        try_idx(point).and_then(|idx| self.tiles.get_mut(idx))
    }

    pub fn is_steppable(&self, point: Point) -> bool {
        match self.tiles.get(row_first_idx(
            point.x.unsigned_abs(),
            point.y.unsigned_abs(),
        )) {
            None => false,
            Some(tile) => *tile == TileType::Floor,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.tiles.get(row_first_idx(x, y)) {
                    None => print!("Unable to draw coordinates {} and {}!", x, y),
                    Some(tile) => match tile {
                        TileType::Floor => {
                            ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x, y, BLACK, BLACK, to_cp437('#'));
                        }
                    },
                }
            }
        }
    }
}
