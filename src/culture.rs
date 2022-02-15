const CULTURE_DIMENSIONS: usize = 8;

use rand::{
    distributions::Standard,
    prelude::{Distribution, Rng},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Culture {
    cult: [f64; CULTURE_DIMENSIONS],
}

impl Default for Culture {
    fn default() -> Self {
        Culture {
            cult: [0.0; CULTURE_DIMENSIONS],
        }
    }
}

impl Culture {
    pub(crate) fn difference(&self, other: &Culture) -> f64 {
        self.cult
            .iter()
            .zip(other.cult.iter())
            .fold(0.0, |dist, (x, y)| dist + (x - y) * (x - y))
            .sqrt()
    }

    pub(crate) fn average(i: impl Iterator<Item = Culture>) -> Self {
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

impl Distribution<Culture> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Culture {
        let mut culture = Culture {
            cult: [0.0; CULTURE_DIMENSIONS],
        };
        for i in 0..CULTURE_DIMENSIONS {
            culture.cult[i] = rng.gen_range(-1.0..1.0);
        }
        culture
    }
}
