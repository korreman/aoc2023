use std::{ops::IndexMut, vec::IntoIter};

use crate::util::{
    graph::{Graph, GraphImpl},
    grid::{pos, Dir4, Grid, Pos, Rot},
    pathfinding::dijkstra,
    queue::SlidingBucketQueue,
};

pub fn run(input: &str) -> (usize, usize) {
    let grid = Grid::parse(input, |_, c| c.to_digit(10).unwrap() as u8);
    let get_edge = |(_, pa): (Dir4, Pos), (_, pb): (Dir4, Pos)| {
        let mut line = pa.line(&pb).unwrap();
        line.next();
        let cost: u8 = line.map(|p| grid[p]).sum();
        Some(cost as usize)
    };
    let is_target = |_, node: (_, Pos)| node.1 == pos(grid.width() - 1, grid.height() - 1);
    // Part 1
    let graph = City {
        grids: [grid.clone(), grid.clone()],
    };

    let part1_a = dijkstra::<_, _, SlidingBucketQueue<30, _>>(
        &graph,
        get_edge,
        is_target,
        (Dir4::E, (pos(0, 0))),
    )
    .unwrap();
    let part1_b = dijkstra::<_, _, SlidingBucketQueue<30, _>>(
        &graph,
        get_edge,
        is_target,
        (Dir4::S, (pos(0, 0))),
    )
    .unwrap();

    // Part 2
    let graph2 = City2 {
        grids: [grid.clone(), grid.clone()],
    };

    let part2_a = dijkstra::<_, _, SlidingBucketQueue<100, _>>(
        &graph2,
        get_edge,
        is_target,
        (Dir4::E, (pos(0, 0))),
    )
    .unwrap();
    let part2_b = dijkstra::<_, _, SlidingBucketQueue<100, _>>(
        &graph2,
        get_edge,
        is_target,
        (Dir4::S, (pos(0, 0))),
    )
    .unwrap();

    (part1_a.min(part1_b), part2_a.min(part2_b))
}

// The challenge here is,
// how do I restrict movement so that you don't walk in the same direction for more than 3 turns?
//
// I'm thinking that maybe positions could include their source direction?
// Checking if a neighbor is valid could then include verifying
// that there haven't been more than two steps in that direction already?
//
// But I'd need four separate grids for each direction.

struct City<T> {
    grids: [Grid<T>; 2],
}

impl<T> std::ops::Index<(Dir4, Pos)> for City<T> {
    type Output = T;

    fn index(&self, (dir, pos): (Dir4, Pos)) -> &Self::Output {
        &self.grids[dir.to_idx() % 2][pos]
    }
}

impl<T> IndexMut<(Dir4, Pos)> for City<T> {
    fn index_mut(&mut self, (dir, pos): (Dir4, Pos)) -> &mut Self::Output {
        &mut self.grids[dir.to_idx() % 2][pos]
    }
}

impl<T> Graph<T> for City<T> {}

impl<T> GraphImpl<T> for City<T> {
    type Node = (Dir4, Pos);

    type Neighbors = IntoIter<Self::Node>;
    fn neighbors(&self, (d, p): Self::Node) -> Self::Neighbors {
        let da = d.rotate(Rot::L);
        let db = da.flip();
        let mut pa = p;
        let mut pb = p;
        let mut res = Vec::new();
        for _ in 0..3 {
            pa = pa.step(da);
            pb = pb.step(db);
            if self.grids[0].contains(pa) {
                res.push((da, pa));
            }
            if self.grids[0].contains(pb) {
                res.push((db, pb));
            }
        }
        res.into_iter()
    }

    type AllNodes = IntoIter<Self::Node>;
    fn nodes(&self) -> Self::AllNodes {
        // We aren't using this
        todo!()
    }

    type Map<U> = City<U>;
    fn map<U, F: Copy + FnMut(&T) -> U>(&self, f: F) -> Self::Map<U> {
        let a = self.grids[0].map(f);
        let b = self.grids[1].map(f);
        City {
            grids: [a, b],
        }
    }
}

struct City2<T> {
    grids: [Grid<T>; 2],
}

impl<T> std::ops::Index<(Dir4, Pos)> for City2<T> {
    type Output = T;

    fn index(&self, (dir, pos): (Dir4, Pos)) -> &Self::Output {
        &self.grids[dir.to_idx() % 2][pos]
    }
}

impl<T> IndexMut<(Dir4, Pos)> for City2<T> {
    fn index_mut(&mut self, (dir, pos): (Dir4, Pos)) -> &mut Self::Output {
        &mut self.grids[dir.to_idx() % 2][pos]
    }
}

impl<T> Graph<T> for City2<T> {}

impl<T> GraphImpl<T> for City2<T> {
    type Node = (Dir4, Pos);

    type Neighbors = IntoIter<Self::Node>;
    fn neighbors(&self, (d, p): Self::Node) -> Self::Neighbors {
        let da = d.rotate(Rot::L);
        let db = da.flip();
        let mut pa = p;
        let mut pb = p;
        let mut res = Vec::new();
        for _ in 0..3 {
            pa = pa.step(da);
            pb = pb.step(db);
        }
        for _ in 0..7 {
            pa = pa.step(da);
            pb = pb.step(db);
            if self.grids[0].contains(pa) {
                res.push((da, pa));
            }
            if self.grids[0].contains(pb) {
                res.push((db, pb));
            }
        }
        res.into_iter()
    }

    type AllNodes = IntoIter<Self::Node>;
    fn nodes(&self) -> Self::AllNodes {
        // We aren't using this
        todo!()
    }

    type Map<U> = City2<U>;
    fn map<U, F: Copy + FnMut(&T) -> U>(&self, f: F) -> Self::Map<U> {
        let a = self.grids[0].map(f);
        let b = self.grids[1].map(f);
        City2 {
            grids: [a, b],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(super::run(input), (102, 94));
    }
}
