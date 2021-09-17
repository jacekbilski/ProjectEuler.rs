pub mod problems {
    use std::fs;

    pub fn solve0102() -> usize {
        #[derive(Debug)]
        struct Vertex {
            x: i32,
            y: i32,
        }

        #[derive(Debug)]
        struct Triangle {
            a: Vertex,
            b: Vertex,
            c: Vertex,
        }

        impl Triangle {
            fn contains_origin(&self) -> bool {
                // idea - wszystkie boki muszą mieć (0,0) po tej samej stronie, ew przechodzić przez
                true
            }
        }

        let triangles_input = fs::read_to_string("data/p102_triangles.txt").unwrap();
        triangles_input.split("\n")
            .filter(|l| l.len() > 0)
            .map(|l| {
                let coords: Vec<i32> = l.split(",")
                    .map(|s| -> i32 { s.parse().unwrap() })
                    .collect();
                Triangle {
                    a: Vertex { x: coords[0], y: coords[1] },
                    b: Vertex { x: coords[2], y: coords[3] },
                    c: Vertex { x: coords[4], y: coords[5] },
                }
            })
            .filter(|t| t.contains_origin())
            .count()
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
