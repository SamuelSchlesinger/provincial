const CULTURE_DIMENSIONS: usize = 16;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Culture {
    cult: [f64; CULTURE_DIMENSIONS],
}

impl Culture {
    pub fn difference(&self, other: &Culture) -> f64 {
        self.cult
            .iter()
            .zip(other.cult.iter())
            .fold(0.0, |dist, (x, y)| dist + (x - y) * (x - y))
            .sqrt()
    }

    pub fn average(i: impl Iterator<Item = Culture>) -> Self {
        let mut avg = [0.0; CULTURE_DIMENSIONS];
        let mut count = 0.0;
        for culture in i {
            count += 1.0;
            for (k, v) in culture.cult.iter().enumerate() {
                avg[k] += v;
            }
        }
        for k in 0..8 {
            avg[k] /= count;
        }
        Culture { cult: avg }
    }
}
