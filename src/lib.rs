use std::collections::HashMap;

pub struct BaseChanger {
    new_base_usize_map: HashMap<usize, char>,
    new_base_char_map: HashMap<char, usize>,
    new_base_size: usize,
    old_base_usize_map: Option<HashMap<usize, char>>,
    old_base_char_map: Option<HashMap<char, usize>>,
    old_base_size: Option<usize>,
}

impl BaseChanger {
    // add code here
    pub fn new(new_base_string: String, old_base_string: Option<String>) -> BaseChanger {
        let (new_base_usize_map, new_base_char_map) =
            BaseChanger::string_to_char_maps(&new_base_string);
        let new_base_size = new_base_string.len();

        let (old_base_usize_map, old_base_char_map, old_base_size) = match old_base_string {
            Some(input_string) => {
                let (usize_map, char_map) = BaseChanger::string_to_char_maps(&input_string);
                (Some(usize_map), Some(char_map), Some(input_string.len()))
            }
            None => (None, None, None),
        };

        return BaseChanger {
            new_base_usize_map,
            new_base_char_map,
            new_base_size,
            old_base_usize_map,
            old_base_char_map,
            old_base_size,
        };
    }

    fn string_to_char_maps(input_string: &String) -> (HashMap<usize, char>, HashMap<char, usize>) {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut usize_map: HashMap<usize, char> = HashMap::new();

        input_string.chars().enumerate().for_each(|(i, c)| {
            char_map.insert(c, i);
            usize_map.insert(i, c);
        });

        return (usize_map, char_map);
    }

    pub fn convert_string(&self, input_string: String) -> Result<String, String> {
        return match self.string_to_int(input_string) {
            Ok(usize_value) => Ok(self.convert_usize(usize_value)),
            Err(e) => Err(e),
        };
    }

    pub fn convert_usize(&self, input_value: usize) -> String {
        let mut curr_value = input_value;
        let mut output_string = String::from("");

        while curr_value > 0 {
            let remainder = curr_value % self.new_base_size;
            let character = self
                .new_base_usize_map
                .get(&remainder)
                .expect("Should not error out, size mismatch");
            output_string = format!("{}{}", character, output_string);
            curr_value = (curr_value - remainder) / self.new_base_size;
            println!("{}", curr_value);
        }

        output_string
    }

    fn string_to_int(&self, input_string: String) -> Result<usize, String> {
        match (self.old_base_size, &self.old_base_char_map) {
            (Some(old_base_size), Some(old_base_char_map)) => input_string
                .chars()
                .rev()
                .enumerate()
                .fold(Ok(0), |maybe_acc, (i, c)| match maybe_acc {
                    Err(e) => Err(e),
                    Ok(acc) => {
                        let int_for_char =
                            BaseChanger::char_to_int(c, old_base_char_map, old_base_size, i + 1);

                        match int_for_char {
                            Ok(int_value) => Ok(acc + int_value),
                            Err(e) => Err(e),
                        }
                    }
                }),
            _ => Err("OldBase undefined can't convert string.".into()),
        }
    }

    fn char_to_int(
        c: char,
        base_char_map: &HashMap<char, usize>,
        base_size: usize,
        power: usize,
    ) -> Result<usize, String> {
        return match base_char_map.get(&c) {
            Some(char_value) => Ok((base_size ^ power) * char_value),
            None => Err(format!("Couldn't find char in old_base: {}", c)),
        };
    }
}

// mod tests {
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

// TODO: binary to base 10
#[test]
fn binary_to_decimal() {
    let base_changer = BaseChanger::new("0123456789".into(), Some("01".into()));
    let output = base_changer.convert_string("1100011".into()).unwrap();
    assert_eq!(output, String::from("99"));
}

// TODO: base 10 to binary without old base
// TODO: base 10 to binary with old base
// TODO: base 10 to hex without old base
// TODO: base 10 to hex with old base
// TODO: base 10 to 🚀🦉 binary without old base
// TODO: base 10 to 🚀🦉 binary with old base
// TODO: 🚀🦉 binary to hexadecimal
// }
