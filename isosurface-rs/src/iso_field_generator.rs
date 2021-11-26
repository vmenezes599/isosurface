use ndarray::{Array, Array3, IntoDimension};
use partial_application::partial;

#[inline(always)]
fn calculate_voxel_value(
    grid_pos: (usize, usize, usize),
    ball_positions: &[(f32, f32, f32)],
) -> f32 {
    ball_positions
        .iter()
        .map(|(posx, posy, posz)| {
            1.0 / (f32::powi(posx - grid_pos.0 as f32, 2)
                + f32::powi(posy - grid_pos.1 as f32, 2)
                + f32::powi(posz - grid_pos.2 as f32, 2))
        })
        .sum()
}

pub fn generate_iso_field(grid_size: usize, ball_positions: &[(f32, f32, f32)]) -> Array3<f32> {
    let voxel_value = partial!(calculate_voxel_value => _, ball_positions);

    Array::from_shape_fn([grid_size; 3].into_dimension(), voxel_value)
}

pub fn generate_iso_field2(
    grid_size: usize,
    ball_positions: &[(f32, f32, f32)],
    iso_surface: &Array3<f32>,
) -> Array3<f32> {
    iso_surface
        .indexed_iter()
        .map(|x: ((usize, usize, usize), &f32)| calculate_voxel_value(x.0, ball_positions))
        .collect::<Array<_, _>>()
        .into_shape((grid_size, grid_size, grid_size))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use pretty_assertions::assert_eq;

    pub const BALL_POS: [(f32, f32, f32); 2] = [(8.5, 8.5, 8.5), (8.5, 17.0, 8.5)];

    pub const GRID_SIZE: usize = 128;

    #[test]
    fn test_generate_iso_surface() {
        let result = generate_iso_field(GRID_SIZE, &BALL_POS);

        assert_eq!(result.shape(), [GRID_SIZE, GRID_SIZE, GRID_SIZE]);
    }

    #[test]
    fn test_generate_iso_surface2() {
        let iso_surface: Array3<f32> = Array::zeros((GRID_SIZE, GRID_SIZE, GRID_SIZE));
        let result = generate_iso_field2(GRID_SIZE, &BALL_POS, &iso_surface);

        assert_eq!(result.shape(), [GRID_SIZE, GRID_SIZE, GRID_SIZE]);
    }

    #[test]
    fn test_compare_two_generators() {
        let result1 = generate_iso_field(GRID_SIZE, &BALL_POS);

        let iso_surface: Array3<f32> = Array::zeros((GRID_SIZE, GRID_SIZE, GRID_SIZE));
        let result2 = generate_iso_field2(GRID_SIZE, &BALL_POS, &iso_surface);

        result1
            .iter()
            .zip(result2.iter())
            .for_each(|(a, b)| assert!(approx_eq!(f32, *a, *b, ulps = 2)));
    }
}
