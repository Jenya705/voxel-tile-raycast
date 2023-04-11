use nalgebra::{Vector3, Vector2};

pub mod na {
    pub use nalgebra::*;
}

#[cfg(all(feature = "f64", feature = "f32"))]
compile_error!("f32 and f64 features can not be enabled together");

#[cfg(not(any(feature = "f64", feature = "f32")))]
compile_error!("f32 or f64 feature should be enabled");

#[cfg(feature = "f64")]
type Real = f64;

#[cfg(feature = "f32")]
type Real = f32;

macro_rules! raycast {
    ($func: ident, $vec_i: ident, $vec_indexes: expr, $stepped_index_func: ident) => {
        pub fn $func(
            origin: $vec_i<Real>,
            dir: $vec_i<Real>,
            max_dir: Real,
            mut func: impl FnMut($vec_i<i32>, $vec_i<Real>, $vec_i<i32>) -> bool,
        ) {
            if dir == $vec_i::zeros() {
                panic!("dir is zero");
            }
            let dir = dir.normalize();
            let mut t = 0.0;
            let mut index = origin.map(|val| val as i32);
            let step = dir.map(|val| val.signum() as i32);
            let t_delta = dir.map(|val| 1.0 / val).abs();
            let dist = $vec_indexes.map(|val| {
                if step[val] > 0 {
                    index[val] as Real + 1.0 - origin[val]
                } else {
                    origin[val] - index[val] as Real
                }
            });
            let mut t_max = $vec_indexes.map(|val| {
                if t_delta[val] < Real::INFINITY {
                    t_delta[val] * dist[val]
                } else {
                    Real::INFINITY
                }
            });

            if !func(
                index,
                $vec_indexes.map(|val| origin[val] + t * dist[val]),
                $vec_i::zeros(),
            ) {
                while t < max_dir {
                    let stepped_index = $stepped_index_func(t_max);

                    index[stepped_index] += step[stepped_index];
                    t = t_max[stepped_index];
                    t_max[stepped_index] += t_delta[stepped_index];

                    if func(
                        index,
                        $vec_indexes.map(|val| origin[val] + t * dist[val]),
                        {
                            let mut hit_norm = $vec_i::zeros();
                            hit_norm[stepped_index] = -step[stepped_index];
                            hit_norm
                        },
                    ) {
                        break;
                    }
                }
            }
        }
    };
}

#[cfg(feature = "voxel")]
fn voxel_stepped_index(t_max: Vector3<Real>) -> usize {
    if t_max.x < t_max.y && t_max.x < t_max.z {
        0
    } else if t_max.y < t_max.z {
        1
    } else {
        2
    }
}

#[cfg(feature = "tile")]
fn tile_stepped_index(t_max: Vector2<Real>) -> usize {
    if t_max.x < t_max.y {
        0
    } else {
        1
    }
}

#[cfg(feature = "voxel")]
raycast!(voxel_raycast, Vector3, Vector3::new(0, 1, 2), voxel_stepped_index);
#[cfg(feature = "tile")]
raycast!(tile_raycast, Vector2, Vector2::new(0, 1), tile_stepped_index);