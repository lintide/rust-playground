#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut p = Point { x: 1, y: 2 };
        let mut_x_ref = &mut p.x;
        let mut_y_ref = &mut p.y;
        
        *mut_y_ref += 1;
        *mut_x_ref += 1;

        println!("Point: {:?}", p);
    }
}