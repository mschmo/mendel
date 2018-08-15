/*!
The `mendel` crate provides a non-sophisticated way to predict probabilities of population
selections, such as the odds of selecting a green ball out of a bag of many different colored
balls or the odds of selecting 2 boys and 1 girl from a classroom.
`mendel` accomplishes not through (arguably trivial) mathematical probability formulas, but
rather via running many simulations on the population selections and recording the results.
*/

mod bag;

pub use bag::Bag;

#[cfg(test)]
mod tests {

    use super::bag::Bag;

    fn close_enough(inp: f64, exp: f64) -> bool {
        // Input is within +/- 1% of the expected result
        inp < exp + 0.01 && inp > exp - 0.01
    }

    #[derive(Clone)]
    struct Region<'a> {
        name: &'a str,
        state: &'a str,
        population: u32 // In thousands
    }

    #[test]
    fn test_bag() {
        let numbers = Bag::from_range(1, 21);
        // 9 out of 20 numbers meet the criteria (45%)
        let result = numbers.one(|v| *v % 3 == 0 || *v % 5 == 0);
        assert!(close_enough(result, 0.45));

        let bag_of_words = Bag::from_vec(vec!["hello", "tomato", "lizard", "golfing"]);
        // 1 out of 5 words meet the criteria (25%)
        let result = bag_of_words.one(|w| *w == "lizard");
        assert!(close_enough(result, 0.25));

        let cities = Bag::from_vec(vec![
            Region { name: "Pittsburgh", state: "PA",  population: 300 },
            Region { name: "Denver", state: "CO", population: 700 },
            Region { name: "State College", state: "PA", population: 42 },
        ]);
        // 1 out of 3 cities meet the criteria
        let result = cities.one(|c| c.state == "PA" && c.population > 200);
        assert!(close_enough(result, 0.33));
    }

    #[derive(Clone, PartialEq, Debug)]
    enum Color {
        Red,
        Blue,
        Green
    }

    #[derive(Clone, Debug)]
    struct Ball {
        color: Color
    }

    #[test]
    fn test_sample() {
        let balls = vec![
            Ball { color: Color::Red },
            Ball { color: Color::Red },
            Ball { color: Color::Green },
            Ball { color: Color::Green },
            Ball { color: Color::Green },
            Ball { color: Color::Blue },
            Ball { color: Color::Blue }
        ];

        let my_bag = Bag::from_vec(balls);
        // (10 / 21) = 47.5% chance of not selecting any blue balls with first 2 picks
        let result = my_bag.sample(2, |balls| {
            for b in balls {
                if b.color == Color::Blue {
                    return false;
                }
            }
            true
        });
        assert!(close_enough(result, 0.475));

        let numbers = Bag::from_range(1, 50);
        let result = numbers.sample(3, |values| {
            let mut s: i32 = 0;
            for v in values {
                s += v;
            }
            s % 2 == 0
        });
        assert!(close_enough(result, 0.5));
    }

    #[test]
    fn test_set_max_sims() {
        let mut bag = Bag::from_range(1, 11);
        bag.set_max_sims(123);
        assert_eq!(bag.max_sims, 123);
    }

}
