pub(crate) const CULTURAL_DIMENSIONS: usize = 8;

use rand::{
    distributions::Standard,
    prelude::{Distribution, Rng},
};

/// Culture represents an ethno-religious identity which can have multiple
/// dimensions. We imagine each of these dimensions as binary axes along which
/// a society might tend towards one taste or its polar opposite. In this vein,
/// we represent these cultures as constant sized vectors with entries between 0
/// and 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Culture {
    cult: [f64; CULTURAL_DIMENSIONS],
}

impl Default for Culture {
    fn default() -> Self {
        Culture {
            cult: [0.0; CULTURAL_DIMENSIONS],
        }
    }
}

impl Culture {
    /// Computes the abstract distance between these two cultures
    pub(crate) fn distance(&self, other: &Culture) -> f64 {
        self.cult
            .iter()
            .zip(other.cult.iter())
            .fold(0.0, |dist, (x, y)| dist + (x - y) * (x - y))
            .sqrt()
    }

    /// Computes a number proportional to how much these cultures agree with each
    /// other
    pub(crate) fn agreement(&self, other: &Culture) -> f64 {
        self.cult
            .iter()
            .zip(other.cult.iter())
            .fold(0.0, |dist, (x, y)| dist + x * y)
            / (CULTURAL_DIMENSIONS as f64)
    }

    pub(crate) fn antagonism(&self, other: &Culture) -> f64 {
        -self.agreement(other)
    }

    /// Averages the cultures as if they were in a melting pot
    pub(crate) fn average(i: impl Iterator<Item = Culture>) -> Self {
        let mut avg = [0.0; CULTURAL_DIMENSIONS];
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

    pub(crate) fn shift_towards(&mut self, other: &Culture, magnitude: f64) {
        let mut direction = [0.0; CULTURAL_DIMENSIONS];
        for i in 0..CULTURAL_DIMENSIONS {
            direction[i] = other.cult[i] - self.cult[i];
        }
        let inv_magnitude = Culture { cult: direction }.distance(&Culture {
            cult: [0.0; CULTURAL_DIMENSIONS],
        });
        for i in 0..CULTURAL_DIMENSIONS {
            direction[i] = direction[i] * magnitude / inv_magnitude;
        }
        for i in 0..CULTURAL_DIMENSIONS {
            self.cult[i] += direction[i];
        }
    }
}

impl Distribution<Culture> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Culture {
        let mut culture = Culture {
            cult: [0.0; CULTURAL_DIMENSIONS],
        };
        for i in 0..CULTURAL_DIMENSIONS {
            culture.cult[i] = rng.gen_range(-1.0..1.0);
        }
        culture
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_cultural_antagonism() {
        let maximalist = Culture {
            cult: [1.0; CULTURAL_DIMENSIONS],
        };
        let minimalist = Culture {
            cult: [-1.0; CULTURAL_DIMENSIONS],
        };
        assert_eq!(maximalist.antagonism(&minimalist), 1.0);
    }

    #[test]
    fn maximum_cultural_agreement() {
        let maximalist = Culture {
            cult: [1.0; CULTURAL_DIMENSIONS],
        };
        let minimalist = Culture {
            cult: [-1.0; CULTURAL_DIMENSIONS],
        };
        assert_eq!(maximalist.antagonism(&minimalist), 1.0);
    }
}
