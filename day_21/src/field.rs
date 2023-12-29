use crate::pos::Pos;

#[derive(Debug, Clone)]
pub struct Field {
    fld: Vec<Vec<bool>>,
    n: usize,
    m: usize,
    start: Pos,
}

impl Field {
    pub fn parse(content: &str) -> Field {
        let mut start = Pos::new(0, 0);
        let fld: Vec<Vec<bool>> = content
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => true,
                        '#' => false,
                        'S' => {
                            start = Pos::new(i, j);
                            true
                        }
                        _ => panic!("Unexpected character {c:?}!"),
                    })
                    .collect()
            })
            .collect();

        let n = fld.len();
        let m = fld.first().unwrap().len();

        Field { fld, n, m, start }
    }

    pub fn load_from_file(file_name: &str) -> Field {
        let content = std::fs::read_to_string(file_name).unwrap();
        Field::parse(&content)
    }

    pub fn start(&self) -> Pos {
        self.start
    }

    pub fn size(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn get(&self, mut pos: Pos) -> bool {
        while pos.i < 0 {
            pos.i += (self.n * 128) as i32;
        }

        while pos.j < 0 {
            pos.j += (self.m * 128) as i32;
        }

        pos.i %= self.n as i32;
        pos.j %= self.m as i32;

        self.fld[pos.i as usize][pos.j as usize]
    }

    pub fn bfs(&self, start: Pos, steps: usize) -> Vec<usize> {
        let mut prev = Vec::<Pos>::new();
        let mut curr = Vec::<Pos>::new();
        let mut next = Vec::<Pos>::new();
        curr.push(start);

        let mut ans = vec![0_usize; steps + 1];

        for cur_dist in 0..steps {
            // #[cfg(debug)]
            // if cur_dist % 100 == 0 {
            // eprintln!(
            // "> {} (+{:.2}%)",
            // curr.len(),
            // (curr.len() as i64 - prev.len() as i64) as f64 / prev.len() as f64 * 100.0
            // );
            // }

            // if (cur_dist & 1) == (steps & 1) {
            ans[cur_dist] = curr.len() + if cur_dist >= 2 { ans[cur_dist - 2] } else { 0 };
            // }

            for pos in curr.iter() {
                pos.neighborhood()
                    .filter(|&nbr| self.get(nbr))
                    .filter(|nbr| prev.binary_search(nbr).is_err())
                    .for_each(|nbr| {
                        next.push(nbr);
                    });
            }

            next.sort();
            next.dedup();

            prev = curr;
            curr = next;
            next = Vec::with_capacity(1024);
        }

        ans[steps] = curr.len() + ans[steps - 2];
        ans
    }
}
