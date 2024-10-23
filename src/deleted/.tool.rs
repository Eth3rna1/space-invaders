/*
    Unspecific functions
*/
use std::thread;
use std::time::Duration;

/// Clears the terminal screen
pub(crate) fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Returns the cursor to the top-left of the screen
pub(crate) fn refresh() {
    print!("\x1B[H");
}

pub(crate) fn sleep(n: f64) {
    thread::sleep(Duration::from_secs_f64(n));
}

fn _index(arr : &[usize], item : &usize) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left <= right {
        if arr[left] == *item { return Some(left) }
        if arr[right] == *item { return Some(right) }
        left += 1;
        right -= 1;
    }
    None
}

pub(crate) fn max_to_min(arr : &[usize]) -> Option<Vec<usize>> {
    if arr.is_empty() { return None }
    let mut new_arr: Vec<usize> = Vec::new();
    let mut clone = arr.to_vec();
    while new_arr.len() != arr.len() {
        let max = arr.iter().max()?;
        clone.remove(_index(arr, &max)?);
        new_arr.push(*max);
    }
    Some(new_arr)
}
