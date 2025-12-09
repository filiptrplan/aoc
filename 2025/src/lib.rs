/// (y,x)
pub type Coord = (usize, usize);
pub fn read_grid_from_str<T>(input: &str, mapper: fn(char) -> T) -> Vec<Vec<T>> {
    let lines = input.split("\n");
    let mut v = Vec::new();
    for line in lines {
        v.push(Vec::new());
        for c in line.chars() {
            v.last_mut().unwrap().push(mapper(c));
        }
    }

    v.retain(|row| !row.is_empty());
    v
}
