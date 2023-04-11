use voxel_tile_raycast::{na::Vector2, tile_raycast};

fn main() {
    tile_raycast(
        Vector2::new(0.0, 1.0),
        Vector2::new(-1.0, 0.5),
        4.0,
        |index, hit_pos, hit_normal| {
            println!("{index:?} {hit_pos:?} {hit_normal:?}");
            false // to continue ray
        },
    )
}
