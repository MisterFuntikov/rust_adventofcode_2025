#[derive(Clone, Copy, Debug)]
struct Pt {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug)]
struct Seg {
    a: Pt,
    is_vertical: bool,
    y1: u64,
    y2: u64,
    x1: u64,
    x2: u64,
}

impl Seg {
    fn new(a: Pt, b: Pt) -> Self {
        let (y1, y2) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        let (x1, x2) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        Self {
            a: a,
            is_vertical: a.x == b.x,
            y1: y1,
            y2: y2,
            x1: x1,
            x2: x2,
        }
    }
}

fn point_in_poly_or_on(p: Pt, poly: &[Pt], segs: &[Seg]) -> bool {
    let mut inside = false;

    for s in segs {
        if (s.is_vertical && p.x == s.a.x && p.y >= s.y1 && p.y <= s.y2)
            || (!s.is_vertical || p.y == s.a.y && p.x >= s.x1 && p.x <= s.x2)
        {
            return true;
        }
    }
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        if a.y != b.y && (a.y > p.y) != (b.y > p.y) {
            let x_cross = a.x as f64 + (p.y - a.y) as f64 * (b.x - a.x) as f64 / (b.y - a.y) as f64;
            if (p.x as f64) < x_cross {
                inside = !inside;
            }
        }
    }
    inside
}

fn poly_intersects_rect_interior(segs: &[Seg], coords: (u64, u64, u64, u64)) -> bool {
    for s in segs {
        if (s.is_vertical
            && coords.0 < s.a.x
            && s.a.x < coords.1
            && std::cmp::max(s.y1, coords.2) < std::cmp::min(s.y2, coords.3))
            || (coords.2 < s.a.y
                && s.a.y < coords.3
                && std::cmp::max(s.x1, coords.0) < std::cmp::min(s.x2, coords.1))
        {
            return true;
        }
    }
    false
}

pub fn day9(path: &String) -> [u64; 2] {
    let content = std::fs::read_to_string(path).unwrap();
    let mut result: [u64; 2] = [0, 0];
    let mut red: Vec<Pt> = vec![];

    for line in content.lines().filter(|l| !l.is_empty()) {
        let r: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        red.push(Pt { x: r[0], y: r[1] });
    }
    let mut segs: Vec<Seg> = Vec::with_capacity(red.len());
    let poly = &red;

    for i in 0..red.len() {
        segs.push(Seg::new(red[i], red[(i + 1) % red.len()]));
    }
    for i in 0..red.len() {
        for j in i + 1..red.len() {
            let area = (red[i].x.abs_diff(red[j].x) + 1) * (red[i].y.abs_diff(red[j].y) + 1);

            if area > result[0] {
                result[0] = area;
            }

            if red[i].x == red[j].x || red[i].y == red[j].y {
                continue;
            }

            let coords = (
                std::cmp::min(red[i].x, red[j].x),
                std::cmp::max(red[i].x, red[j].x),
                std::cmp::min(red[i].y, red[j].y),
                std::cmp::max(red[i].y, red[j].y),
            );

            let (c, d) = (
                Pt {
                    x: red[i].x,
                    y: red[j].y,
                },
                Pt {
                    x: red[j].x,
                    y: red[i].y,
                },
            );

            if !point_in_poly_or_on(c, poly, &segs)
                || !point_in_poly_or_on(d, poly, &segs)
                || poly_intersects_rect_interior(&segs, coords)
            {
                continue;
            }

            let area = (coords.1.abs_diff(coords.0) + 1) * (coords.3.abs_diff(coords.2) + 1);
            if area > result[1] {
                result[1] = area;
            }
        }
    }

    result
}
