use rand::Rng;
use std::array;
use std::io::{self, Write};

const READ_STDIO_ERR_MSG: &'static str = "failed to read stdin.";
const INPUT_ERR_MSG: &'static str = "invalid input; format is \"<u8> <u8> <u8> <u8>\".";

fn main() -> Result<(), String> {
    let mut rng = rand::rng();
    let secret: [u8; 4] = array::from_fn(|_| rng.random_range(1..=9));

    loop {
        let mut s = String::new();
        print!("> ");
        io::stdout().flush().map_err(|_| READ_STDIO_ERR_MSG)?;
        io::stdin()
            .read_line(&mut s)
            .map_err(|_| READ_STDIO_ERR_MSG)?;
        let s = s.trim();
        let guess: Vec<u8> = s
            .split(' ')
            .map(|c| c.parse::<u8>())
            .filter_map(Result::ok)
            .collect();
        if guess.len() != 4 {
            println!("{}", INPUT_ERR_MSG);
            continue;
        }
        let res = calc_green_and_yellow(&[guess[0], guess[1], guess[2], guess[3]], &secret);
        println!("{}", res);
        if res == String::from("🟩🟩🟩🟩") {
            break;
        }
    }
    Ok(())
}

fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String {
    let mut res = ['⬜'; 4];
    let mut guessed = [false; 4];
    for i in 0..4 {
        if guess[i] == secret[i] {
            res[i] = '🟩';
            guessed[i] = true;
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            if guess[i] == secret[j] && !guessed[j] && res[i] == '⬜' {
                res[i] = '🟨';
                guessed[j] = true;
                break;
            }
        }
    }

    return res.iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_wrong() {
        assert_eq!(
            &calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
            "⬜⬜⬜⬜"
        );
    }

    #[test]
    fn all_green() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
            "🟩🟩🟩🟩"
        );
    }

    #[test]
    fn one_wrong() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
            "🟩🟩🟩⬜"
        );
    }

    #[test]
    fn all_yellow() {
        assert_eq!(
            &calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
            "🟨🟨🟨🟨"
        );
    }

    #[test]
    fn one_wrong_but_duplicate() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
            "🟩🟩🟩⬜"
        );
    }

    #[test]
    fn one_right_others_duplicate() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
            "🟩⬜⬜⬜"
        );
    }

    #[test]
    fn two_right_two_swapped() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
            "🟨🟩🟩🟨"
        );
    }

    #[test]
    fn two_wrong_two_swapped() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
            "🟨⬜⬜🟨"
        );
    }

    #[test]
    fn a_bit_of_everything() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
            "🟩⬜🟨🟨"
        );
    }

    #[test]
    fn two_in_guess_one_in_secret() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 3], &[3, 9, 9, 9]),
            "⬜⬜🟨⬜"
        );
    }

    #[test]
    fn two_in_secret_one_in_guess() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
            "⬜⬜🟨⬜"
        );
    }
}
