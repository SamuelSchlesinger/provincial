use crate::culture::Culture;

const MAX_LIFESPAN: usize = 120;

#[derive(Debug, PartialEq)]
pub(crate) struct Demographics {
    communities: Vec<Community>,
    /// NB: Cached, be careful.
    total_population: u64,
    /// NB: Cached, be careful.
    average_culture: Culture,
}

impl Demographics {
    pub(crate) fn new(communities: impl Iterator<Item = Community>) -> Self {
        let communities: Vec<Community> = communities.collect();
        let average_culture = Culture::average(communities.iter().map(|x| x.culture));
        let total_population = communities.iter().map(|x| x.population).sum();

        Demographics {
            communities,
            total_population,
            average_culture,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CommunityId(u32);

#[derive(Debug, PartialEq)]
pub(crate) struct Community {
    id: CommunityId,
    culture: Culture,
    /// NB: Cached, be careful.
    population: u64,
    /// Number of people of each age
    ages: Ages,
}

impl Community {
    pub(crate) fn new(id: CommunityId, culture: Culture, ages: Ages) -> Self {
        Community {
            id,
            culture,
            population: ages.counts.iter().sum(),
            ages,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Ages {
    counts: [u64; MAX_LIFESPAN],
    birth_rates: [f64; MAX_LIFESPAN],
    death_rates: [f64; MAX_LIFESPAN],
}

impl Ages {
    pub(crate) fn new(
        counts: [u64; MAX_LIFESPAN],
        birth_rates: [f64; MAX_LIFESPAN],
        death_rates: [f64; MAX_LIFESPAN],
    ) -> Self {
        Ages {
            counts,
            birth_rates,
            death_rates,
        }
    }

    pub(crate) fn step_year(&mut self) {
        let mut counts: [u64; MAX_LIFESPAN] = [0; MAX_LIFESPAN];
        counts[0] = self
            .counts
            .iter()
            .zip(self.birth_rates.iter())
            .map(|(population, birth_rate)| (*population as f64 * birth_rate) as u64)
            .fold(0, |acc, next| acc.saturating_add(next));
        for i in 1..MAX_LIFESPAN {
            counts[i] = self.counts[i - 1]
                .saturating_sub((self.counts[i - 1] as f64 * self.death_rates[i - 1]) as u64);
        }
        self.counts = counts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_population_stays_zero() {
        let mut ages = Ages::new([0; MAX_LIFESPAN], [0.0; MAX_LIFESPAN], [0.0; MAX_LIFESPAN]);
        let cloned_ages = ages.clone();
        for _ in 0..(MAX_LIFESPAN + 1) {
            ages.step_year();
            assert_eq!(cloned_ages, ages);
        }
    }

    #[test]
    fn high_birth_rates_quickly_saturate_population() {
        let mut ages = Ages::new([1; MAX_LIFESPAN], [2.0; MAX_LIFESPAN], [0.0; MAX_LIFESPAN]);
        for _ in 0..1000 {
            ages.step_year();
        }
        assert_eq!(
            ages,
            Ages::new(
                [u64::MAX; MAX_LIFESPAN],
                [2.0; MAX_LIFESPAN],
                [0.0; MAX_LIFESPAN]
            )
        );
    }

    #[test]
    fn reasonable_rates_dont_quickly_saturate_or_destroy_population() {
        let mut ages = Ages::new(
            [100; MAX_LIFESPAN],
            [1.05; MAX_LIFESPAN],
            [0.007; MAX_LIFESPAN],
        );
        for _ in 0..100000 {
            ages.step_year();
        }
        assert_ne!(
            ages,
            Ages::new(
                [u64::MAX; MAX_LIFESPAN],
                [1.05; MAX_LIFESPAN],
                [0.007; MAX_LIFESPAN]
            )
        );
        assert_ne!(
            ages,
            Ages::new(
                [0; MAX_LIFESPAN],
                [1.05; MAX_LIFESPAN],
                [0.007; MAX_LIFESPAN]
            )
        );
    }
}
