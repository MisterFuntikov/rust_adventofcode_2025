struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, edge: &(i64, usize, usize)) -> bool {
        let (mut i, mut j) = (self.find(edge.1), self.find(edge.2));
        if i == j {
            return false;
        }
        if self.size[i] < self.size[j] {
            std::mem::swap(&mut i, &mut j);
        }
        (self.parent[j], self.size[i]) = (i, self.size[i] + self.size[j]);
        true
    }

    fn sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.parent.len()];
        for i in 0..self.parent.len() {
            sizes[self.find(i)] += 1;
        }
        sizes.into_iter().filter(|&x| x > 0).collect()
    }
}

pub fn day8(path: &String) -> [u64; 2] {
    let content = std::fs::read_to_string(path).unwrap();
    let pairs = if path.contains("example") { 10 } else { 1000 };
    let mut result: [u64; 2] = [0, 0];
    let mut pts: Vec<Vec<i64>> = Vec::new();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();

    for line in content.lines().filter(|l| !l.is_empty()) {
        pts.push(line.split(',').map(|x| x.parse().unwrap()).collect());
    }
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            let r = (pts[i][0] - pts[j][0]).pow(2)
                + (pts[i][1] - pts[j][1]).pow(2)
                + (pts[i][2] - pts[j][2]).pow(2);
            edges.push((r, i, j));
        }
    }
    edges.sort_by_key(|x| x.0);

    let mut dsu = DSU::new(pts.len());

    for i in 0..edges.len() {
        if i == pairs {
            let mut sizes = dsu.sizes();
            sizes.sort_by_key(|&x| std::cmp::Reverse(x));
            result[0] = (sizes[0] * sizes[1] * sizes[2]) as u64;
        }
        if dsu.union(&edges[i]) {
            let find = dsu.find(edges[i].1);
            if dsu.size[find] == pts.len() {
                result[1] = (pts[edges[i].1][0] * pts[edges[i].2][0]) as u64;
                break;
            }
        }
    }

    result
}
