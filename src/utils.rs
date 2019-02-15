//use std::ops::Mul;
//use std::ops::Add;

pub fn map_t_of_range_a_to_range_b(
    t: f32,
    range_a_start: f32,
    range_a_end: f32,
    range_b_start: f32,
    range_b_end: f32,
) -> f32 {
    let slope = (range_b_end - range_b_start) / (range_a_end - range_a_start);
    range_b_start + slope * (t - range_a_start)
}

pub trait Interpolate {
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

// This can be used once specialization is stable
//impl<T: Mul<f32, Output = T> + Add<Output = T> + Copy> Interpolate for T {
//    fn interpolate(&self, other: &Self, t: f32) -> Self {
//        *self * (1.0 - t) + *other * t
//    }
//}

impl Interpolate for u8 {
    fn interpolate(&self, other: &Self, t: f32) -> u8 {
        (f32::from(*self) * (1.0 - t) + f32::from(*other) * t) as u8
    }
}

pub fn get_right_neighbour_index_wrapping(current_index: usize, collection_size: usize) -> usize {
    get_index_wrapping(
        current_index as isize + 1 as isize,
        collection_size as isize,
    ) as usize
}

pub fn get_left_neighbour_index_wrapping(current_index: usize, collection_size: usize) -> usize {
    get_index_wrapping(current_index as isize - 1, collection_size as isize) as usize
}

pub fn get_index_wrapping(index: isize, upper_bound: isize) -> isize {
    if index >= 0 {
        index % upper_bound
    } else {
        (index % upper_bound + upper_bound) % upper_bound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index_wrapping() {
        assert_eq!(0, get_index_wrapping(0, 10));
        assert_eq!(0, get_index_wrapping(10, 10));
        assert_eq!(5, get_index_wrapping(5, 10));
        assert_eq!(9, get_index_wrapping(19, 10));
        assert_eq!(0, get_index_wrapping(-10, 10));
    }

    #[test]
    fn test_get_right_neighbour_index_wrapping() {
        assert_eq!(1, get_right_neighbour_index_wrapping(0, 4));
        assert_eq!(0, get_right_neighbour_index_wrapping(3, 4));
    }

    #[test]
    fn test_get_left_neighbour_index_wrapping() {
        assert_eq!(2, get_left_neighbour_index_wrapping(3, 4));
        assert_eq!(3, get_left_neighbour_index_wrapping(0, 4));
    }
}
