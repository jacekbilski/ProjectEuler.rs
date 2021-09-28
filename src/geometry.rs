pub type Coords = i32;

#[derive(Debug)]
pub struct Vertex {
    pub x: Coords,
    pub y: Coords,
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
    use rstest::*;

    use crate::geometry::{Coords, Triangle, Vertex};

    #[rstest]
    #[case(- 1, - 1, 1, - 1, 0, 1, true)]
    fn contains_origin(
        #[case] ax: Coords, #[case] ay: Coords,
        #[case] bx: Coords, #[case] by: Coords,
        #[case] cx: Coords, #[case] cy: Coords,
        #[case] contains: bool) {
        let t = Triangle {
            a: Vertex { x: ax, y: ay },
            b: Vertex { x: bx, y: by },
            c: Vertex { x: cx, y: cy },
        };
        assert_eq!(t.contains_origin(), contains);
    }
}
