pub mod problems {
    use std::fs;

    pub fn solve0102() -> u32 {
        #[derive(Debug)]
        struct Vertex {
            x: i32,
            y: i32,
        }

        impl Vertex {
            fn parse(x_str: &str, y_str: &str) -> Self {
                let x: i32 = x_str.parse().unwrap();
                let y: i32 = y_str.parse().unwrap();
                Vertex { x, y }
            }
        }

        #[derive(Debug)]
        struct Triangle {
            a: Vertex,
            b: Vertex,
            c: Vertex,
        }

        impl Triangle {
            fn contains_origin(&self) -> bool {
                true
            }
        }

        let triangles_input = fs::read_to_string("data/p102_triangles.txt").unwrap();
        let lines: Vec<&str> = triangles_input.split("\n").collect();
        for l in lines {
            if l.len() > 0 {
                let coords: Vec<&str> = l.split(",").collect();
                let v1 = Vertex::parse(coords[0], coords[1]);
                let v2 = Vertex::parse(coords[2], coords[3]);
                let v3 = Vertex::parse(coords[4], coords[5]);
                let triangle = Triangle { a: v1, b: v2, c: v3 };
                println!("A triangle: {:?}", triangle);
            }
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::*;

    #[test]
    fn problem0102() {
        let result = solve0102();
        println!("Problem 0102: {}", result);
        assert_eq!(result, 1);
    }
}
