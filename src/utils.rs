pub fn clamp(x: usize, min: usize, max: usize) -> usize {
    if x < min { return min }
    if x > max { return max }
    else { return x }
}
