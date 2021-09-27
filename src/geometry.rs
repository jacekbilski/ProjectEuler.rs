#[derive(Debug)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Triangle {
    pub fn contains_origin(&self) -> bool {
        // idea - wszystkie boki muszą mieć (0,0) po tej samej stronie, ew przechodzić przez

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Triangle, Vertex};

    #[test]
    fn contains_origin() {
        let t = Triangle {
            a: Vertex { x: -1, y: -1 },
            b: Vertex { x: 1, y: -1 },
            c: Vertex { x: 0, y: 1 },
        };
        assert_eq!(t.contains_origin(), true);
    }
}
