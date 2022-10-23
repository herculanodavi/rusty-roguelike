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

pub fn row_first_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x).unsigned_abs() as usize
}

pub fn try_idx(point: Point) -> Option<usize> {
    if is_in_bounds(point) {
        Some(row_first_idx(point.x, point.y))
    } else {
        None
    }
}

pub fn is_in_bounds(point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn tile_at(&self, point: Point) -> Option<&TileType> {
        try_idx(point).and_then(|idx| self.tiles.get(idx))
    }

    pub fn mut_tile_at(&mut self, point: Point) -> Option<&mut TileType> {
        try_idx(point).and_then(|idx| self.tiles.get_mut(idx))
    }

    pub fn is_steppable(&self, point: Point) -> bool {
        match self.tiles.get(row_first_idx(point.x, point.y)) {
            None => false,
            Some(tile) => *tile == TileType::Floor,
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if let Some(tile) = self.tile_at(Point::new(x, y)) {
                    match tile {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }
}
