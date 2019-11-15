use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct BaseChanger {
    new_base_usize_map: HashMap<usize, char>,
    new_base_char_map: HashMap<char, usize>,
    old_base_usize_map: Option<HashMap<usize, char>>,
    old_base_char_map: Option<HashMap<char, usize>>
}

impl BaseChanger {
    // add code here
    pub fn new(new_base_string: String, old_base_string: Option<String>) -> BaseChanger {
        let (new_base_usize_map, new_base_char_map) =
            BaseChanger::string_to_char_maps(new_base_string);

        let (old_base_usize_map, old_base_char_map) = match old_base_string {
            Some(input_string) => {
                let (usize_map, char_map) = BaseChanger::string_to_char_maps(input_string);
                (Some(usize_map), Some(char_map))
            },
            None => (None, None),
        };

        return BaseChanger {
            new_base_usize_map,
            new_base_char_map,
            old_base_usize_map,
            old_base_char_map,
        };
    }

    fn string_to_char_maps(input_string: String) -> (HashMap<usize, char>, HashMap<char, usize>) {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut usize_map: HashMap<usize, char> = HashMap::new();

        input_string.chars().enumerate().for_each(|(i, c)| {
            char_map.insert(c, i);
            usize_map.insert(i, c);
        });

        return (usize_map, char_map);
    }
}
