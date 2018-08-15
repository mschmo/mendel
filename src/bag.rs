extern crate rand;

use std::env;
use std::fmt::Debug;
use std::iter::FromIterator;
use self::rand::{thread_rng, seq, Rng};

const MAX_SIMS: u32 = 100_000;

/// The `Bag` struct. The main collection type for holding populations of things.
pub struct Bag<T: Clone> {
    pub items: Vec<T>,
    pub max_sims: u32
}

fn get_default_max_sims() -> u32 {
    match env::var("MENDEL_MAX_SIMS") {
        Ok(val) => val.parse::<u32>().unwrap(),
        Err(_) => MAX_SIMS
    }
}

impl<T: Clone> Bag<T> {

    /// Constructs a new `Bag<T>` from range.
    ///
    /// [`min`, `max`)
    ///
    /// # Examples
    ///
    /// Generate a new `Bag<i32>` with numbers 1 through 10:
    ///
    /// ```
    /// use mendel::Bag;
    ///
    /// let my_number_bag = Bag::from_range(1, 11);
    /// ```
    pub fn from_range(min: i32, max: i32) -> Self where
        Vec<T>: FromIterator<i32> {
        // TODO: Add shuffle option
        let items: Vec<T> = (min..max).collect();
        Bag { items, max_sims: get_default_max_sims() }
    }

    /// Constructs a new `Bag<T>` from a vector of items.
    ///
    /// # Examples
    ///
    /// Generate a new `Bag<&str>`:
    ///
    /// ```
    /// use mendel::Bag;
    ///
    /// let animals = vec!["spider", "fish", "tiger", "pigeon"];
    /// let animal_bag = Bag::from_vec(animals);
    /// ```
    pub fn from_vec(v: Vec<T>) -> Self {
        let items: Vec<T> = v.clone();
        Bag { items, max_sims: get_default_max_sims() }
    }

    /// Predicts probability of criteria being met for the first random item grabbed from the bag.
    ///
    /// # Examples
    ///
    /// Odds of selecting an even number from 1 - 10. Assert factors in +/- 1%:
    ///
    /// ```
    /// use mendel::Bag;
    ///
    /// let my_bag = Bag::from_range(1, 11);
    /// let odds_of_even = my_bag.one(|v| v % 2 == 0);
    /// assert!(0.49 < odds_of_even && odds_of_even < 0.51);
    /// ```
    pub fn one<F>(&self, f: F) -> f64 where
        F: Fn(&T) -> bool {
        let mut picks_in_favor: u32 = 0;
        for _ in 0..self.max_sims {
            let idx = thread_rng().gen_range(0, self.items.len());
            let item = &self.items[idx];
            if f(item) {
                picks_in_favor += 1;
            }
        }
        picks_in_favor as f64 / MAX_SIMS as f64
    }

    /// Predicts probability of criteria being met for the first `sample_size` random items grabbed from the bag.
    ///
    /// # Examples
    ///
    /// Odds of getting a 2 in your first 3 picks from a list of numbers 1 - 10:
    ///
    /// ```
    /// use mendel::Bag;
    ///
    /// let my_bag = Bag::from_range(1, 11);
    /// let odds_of_two = my_bag.sample(3, |values| {
    ///     for v in values {
    ///         if *v == 2 {
    ///             return true;
    ///         }
    ///     }
    ///     false
    /// });
    /// assert!(0.29 < odds_of_two && odds_of_two < 0.31);
    /// ```
    pub fn sample<F>(&self, sample_size: usize, f: F) -> f64 where
        T: Debug,
        F: Fn(Vec<&T>) -> bool {
        let mut picks_in_favor: u32 = 0;
        let mut rng = thread_rng();
        let items_clone = self.items.clone();
        for _ in 0..self.max_sims {
            let sample = seq::sample_iter(&mut rng, &items_clone, sample_size).unwrap();
            if f(sample) {
                picks_in_favor += 1;
            }
        }
        picks_in_favor as f64 / MAX_SIMS as f64
    }

    /// Set the Bag's maximum amount of simulations to run when generating probabilities.
    ///
    /// The default `max_sims` is set by either the MENDEL_MAX_SIMS environment variable value,
    /// or if that env var doesn't exist than it defaults to 100,000. In the future this may
    /// be a field that can be set during initialization of the `Bag` struct.
    ///
    /// # Examples
    ///
    /// Set the max simulations to 10 thousand:
    ///
    /// ```
    /// use mendel::Bag;
    ///
    /// let mut my_bag = Bag::from_range(1, 11);
    /// my_bag.set_max_sims(10_000);
    /// assert!(my_bag.max_sims == 10_000);
    /// ```
    pub fn set_max_sims(&mut self, max_sims: u32) -> () {
        self.max_sims = max_sims;
    }
}
