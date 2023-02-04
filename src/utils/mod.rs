pub mod bresenham;
pub mod edge;
pub mod file_load;

pub fn get_prev(i: usize, size: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        size - 1
    }
}

pub fn get_next(i: usize, size: usize) -> usize {
    (i + 1) % size
}
