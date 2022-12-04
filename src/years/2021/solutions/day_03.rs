use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day03 {
    data: String,
}

impl Day03 {
    pub fn new() -> Result<Day03> {
        let data = Day03::get_input(2021, 3, false)?;

        Ok(Self { data })
    }

    fn get_amounts_for_column(&self, col: usize) -> (u32, u32) {
        let mut amount: (u32, u32) = (0, 0);

        self.data.lines().for_each(|line| {
            let mut chars = line.chars();
            if let Some(c) = chars.nth(col) {
                if c == '0' {
                    amount.0 += 1;
                } else {
                    amount.1 += 1;
                }
            }
        });

        amount
    }

    fn string_to_u32(&self, string: &str) -> Result<u32> {
        Ok(u32::from_str_radix(string, 2)?)
    }
}

impl Day for Day03 {
    fn a(&self) -> Result<String> {
        let line_len = self.data.lines().next().unwrap().len();
        let mut bits_count: Vec<(u32, u32)> = Vec::new();
        let mut gamma_bits = String::new();

        for col in 0..line_len {
            bits_count.push(self.get_amounts_for_column(col));
        }

        for count in bits_count {
            if count.0 > count.1 {
                gamma_bits.push('0');
            } else {
                gamma_bits.push('1');
            }
        }

        let gamma = self.string_to_u32(gamma_bits.as_str())?;
        let epsilon = !gamma ^ 0xFFFFF000;

        let solution = format!(
            "Gamma: {:b} ({}) | Epsilon: {:b} ({}) | Power consumption: {}",
            gamma,
            gamma,
            epsilon,
            epsilon,
            gamma * epsilon
        );

        Ok(solution)
    }

    //TODO: This is not solved yet!
    fn b(&self) -> Result<String> {
        let line_len = self.data.lines().next().unwrap().len();
        let mut oxygen_generator_rating_filter = self.data.lines().collect::<Vec<&str>>();
        let mut co2_scrubber_rating_filter = oxygen_generator_rating_filter.clone();
        let mut oxygen_generator_rating: u32 = 0;
        let mut co2_scrubber_rating: u32 = 0;

        for col in 0..line_len {
            let amounts = self.get_amounts_for_column(col);
            let most_common = if amounts.0 > amounts.1 { '0' } else { '1' };

            oxygen_generator_rating_filter = oxygen_generator_rating_filter
                .iter()
                .filter(|line| {
                    let c = line.chars().nth(col).unwrap();
                    c == most_common
                })
                .copied()
                .collect::<Vec<&str>>();

            co2_scrubber_rating_filter = co2_scrubber_rating_filter
                .iter()
                .filter(|line| {
                    let c = line.chars().nth(col).unwrap();
                    c != most_common
                })
                .copied()
                .collect::<Vec<&str>>();

            if oxygen_generator_rating_filter.len() == 1 {
                oxygen_generator_rating =
                    self.string_to_u32(oxygen_generator_rating_filter.first().unwrap())?;
            }

            if co2_scrubber_rating_filter.len() == 1 {
                co2_scrubber_rating =
                    self.string_to_u32(co2_scrubber_rating_filter.first().unwrap())?;
            }
        }

        // dbg!(oxygen_generator_rating_filter);
        // dbg!(co2_scrubber_rating_filter);

        let solution = format!("Oxygen generator rating: {} ({:b}) | CO2 scrubber rating: {} ({:b}) | Life support rating: {}", oxygen_generator_rating, oxygen_generator_rating, co2_scrubber_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating);

        Ok(solution)
    }

    fn get_title(&self) -> &str {
        "--- Day 3: Binary Diagnostic ---"
    }
}
