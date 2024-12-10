struct DS {
    parents: Vec<usize>,
    rank: Vec<u32>,
    h: usize,
    w: usize
}

// y, x
type Coord = (usize, usize);

impl DS {
    fn new(w: usize, h:usize) -> DS {
        let size = h * w;
        DS {
            parents: (0..size).collect(),
            rank: vec![1; size],
            h,
            w
        }
    }

    fn to_idx(&self, i: usize, j: usize) -> usize {
        i * self.w + j
    }

    fn from_idx(&self, idx: usize) -> Coord {
        (idx / self.w, idx % self.w)
    }

    fn find_idx(&mut self, idx: usize) -> usize {
        if self.parents[idx] == idx {
            return idx;
        } else {
            let res = self.find_idx(self.parents[idx]);
            self.parents[idx] = res;
            return res;
        }
    }
    
    fn find(&mut self, i: usize, j: usize) -> Coord {
        let idx = self.to_idx(i, j);
        let res = self.find_idx(idx);
        self.from_idx(res)
    }

    fn union_idx(&mut self, i: usize, j: usize) {
        let idx1 = self.find_idx(i);
        let idx2 = self.find_idx(j);

        if idx1 == idx2 {
            return;
        }

        let rank1 = self.rank[idx1];
        let rank2 = self.rank[idx2];


        if rank1 < rank2 {
            self.parents[idx1] = idx2;
        } else if rank2 < rank1 {
            self.parents[idx2] = idx1;
        } else {
            self.parents[idx1] = idx2;
            self.rank[idx2] += 1;
        }
    }

    fn union(&mut self, i: Coord, j: Coord) {
        let idx1 = self.to_idx(i.0, i.1);
        let idx2 = self.to_idx(j.0, j.1);
        self.union_idx(idx1, idx2);
    }
}


fn main() {

}