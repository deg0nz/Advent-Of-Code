use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day01 {
    input: String,
}

impl Day01 {
    pub fn new() -> Result<Day01> {
        let input = Day01::get_input(2023, 1, false)?;

        Ok(Self { input })
    }

    fn get_digits_from_line(line: &str) -> Vec<(usize, char)> {
        let numbers_char = line
            .chars()
            .enumerate()
            .filter(|(_i, c)| {
                if let Some(_num) = c.to_digit(10) {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<(usize, char)>>();

        numbers_char
    }

    fn get_calibration_value_from_line_only_digits(line: &str) -> u32 {
        let mut calibration_value_string = String::from("");
        let numbers_char = Day01::get_digits_from_line(line)
            .iter()
            .map(|(_i, c)| *c)
            .collect::<Vec<char>>();

        if numbers_char.len() == 1 {
            let digit = numbers_char.iter().next().unwrap();
            calibration_value_string.push(*digit);
            calibration_value_string.push(*digit);
        } else if numbers_char.len() > 1 {
            let first_digit = numbers_char.iter().next().unwrap();
            let last_digit = numbers_char.iter().last().unwrap();
            calibration_value_string.push(*first_digit);
            calibration_value_string.push(*last_digit);
        } else {
            calibration_value_string.push('0');
        }

        let calibration_value = calibration_value_string.parse::<u32>().unwrap();

        calibration_value
    }

    fn get_indexed_digits_from_digit_words_in_line(line: &str) -> Vec<(usize, char)> {
        let digit_words = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut digits = digit_words
            .iter()
            .enumerate()
            .map(|(digit_word_pos, digit_word)| {
                line.match_indices(digit_word)
                    .map(|(i, _digit_word)| {
                        (
                            i,
                            char::from_digit((digit_word_pos + 1) as u32, 10).unwrap(),
                        )
                    })
                    .collect::<Vec<(usize, char)>>()
            }) // Here, we have an iterator of iterators, so we flatten
            .flatten()
            .collect::<Vec<(usize, char)>>();

        digits.sort_by(|a, b| a.0.cmp(&b.0));

        digits
    }

    fn get_calibration_value_from_line_digits_and_words(line: &str) -> u32 {
        let digit_words = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let values_words = Day01::get_indexed_digits_from_digit_words_in_line(line);
        let values_numbers = Day01::get_digits_from_line(line);

        let first: char;
        let last: char;

        if values_words.len() == 0 {
            return Day01::get_calibration_value_from_line_only_digits(line);
        }

        if values_numbers.len() == 0 && values_words.len() > 0 {
            first = values_words.iter().next().unwrap().1;
            last = values_words.iter().last().unwrap().1;
        } else if values_numbers.len() > 0 && values_words.len() > 0 {
            let first_words = values_words.iter().next().unwrap();
            let first_numbers = values_numbers.iter().next().unwrap();

            if first_numbers.0 < first_words.0 {
                first = first_numbers.1;
            } else {
                first = first_words.1;
            }

            let last_words = values_words.last().unwrap();
            let last_numbers = values_numbers.last().unwrap();

            let word_len = digit_words
                .get((last_words.1.to_digit(10).unwrap() - 1) as usize)
                .unwrap()
                .len();

            if last_words.0 + (word_len) > last_numbers.0 {
                last = last_words.1;
            } else {
                last = last_numbers.1;
            }
        } else {
            return 0;
        }

        let mut value_string = String::from("");
        value_string.push(first);
        value_string.push(last);

        let calibration_value = value_string.parse::<u32>().unwrap();

        calibration_value
    }
}

impl Day for Day01 {
    fn a(&self) -> Result<String> {
        let mut sum = 0;

        self.input.lines().for_each(|line| {
            let calibration_value = Day01::get_calibration_value_from_line_only_digits(line);
            sum += calibration_value;
        });

        Ok(sum.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut sum = 0;

        self.input.lines().for_each(|line| {
            let calibration_value = Day01::get_calibration_value_from_line_digits_and_words(line);
            sum += calibration_value
        });

        Ok(sum.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 1: Trebuchet?! ---"
    }
}
