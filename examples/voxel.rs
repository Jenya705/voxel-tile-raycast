use voxel_tile_raycast::{na::Vector3, voxel_raycast};

fn main() {
    voxel_raycast(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, -1.0, 0.5),
        3.0,
        |index, hit_pos, hit_normal| {
            println!("{index:?} {hit_pos:?} {hit_normal:?}");
            false // to continue ray
        }
    )
}