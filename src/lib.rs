use std::collections::HashMap;

/// Error that can occur during converting
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertError {
    UnknownCharacter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Icao {
    /// The ICAO alphabet
    alphabet: HashMap<char, &'static str>,
}

impl Icao {
    /// Create ICAO structure
    pub fn new() -> Self {
        Self {
            alphabet: Self::create_alphabet(),
        }
    }

    /// Convert text into ICAO alphabet. Text must be in uppercase.
    pub fn convert(&self, text: &str) -> Result<String, ConvertError> {
        
        let mut result = String::from("");
        
        for c in text.chars() {
            match self.alphabet.get(&c) {
                Some(word) => {
                    result = result + word;
                    result = result + " ";
                }
                None => {
                    return Err(ConvertError::UnknownCharacter);
                }
            }
        }
        Ok(result)
    }
    
    /// Initialize hash map of characters to ICAO alphabet.
    fn create_alphabet() -> HashMap<char, &'static str> {
        let mut alphabet: HashMap<char, &str> = HashMap::new();
        alphabet.insert(' ', " "); // Space
        alphabet.insert('A', "Alfa");
        alphabet.insert('B', "Bravo");
        alphabet.insert('C', "Charlie");
        alphabet.insert('D', "Delta");
        alphabet.insert('E', "Echo");
        alphabet.insert('F', "Foxtrot");
        alphabet.insert('G', "Golf");
        alphabet.insert('H', "Hotel");
        alphabet.insert('I', "India");
        alphabet.insert('J', "Juliett");
        alphabet.insert('K', "Kilo");
        alphabet.insert('L', "Lima");
        alphabet.insert('M', "Mike");
        alphabet.insert('N', "November");
        alphabet.insert('O', "Oscar");
        alphabet.insert('P', "Papa");
        alphabet.insert('Q', "Quebec");
        alphabet.insert('R', "Romeo");
        alphabet.insert('S', "Sierra");
        alphabet.insert('T', "Tango");
        alphabet.insert('U', "Uniform");
        alphabet.insert('V', "Victor");
        alphabet.insert('W', "Whiskey");
        alphabet.insert('X', "X-Ray");
        alphabet.insert('Y', "Yankee");
        alphabet.insert('Z', "Zulu");
        alphabet.insert('0', "Zero");
        alphabet.insert('1', "One");
        alphabet.insert('2', "Two");
        alphabet.insert('3', "Three");
        alphabet.insert('4', "Four"); // Fower
        alphabet.insert('5', "Five");
        alphabet.insert('6', "Six");
        alphabet.insert('7', "Seven");
        alphabet.insert('8', "Eight");
        alphabet.insert('9', "Nine"); // Niner
        alphabet.insert('.', "Stop"); // End sentence
        alphabet
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_callsign() {
        let icao = Icao::new();
        
        let result = icao.convert("LH26EDF");
        let callsign = result.unwrap();
        assert_eq!(callsign, "Lima Hotel Two Six Echo Delta Foxtrot ");
    }
    
    #[test]
    fn check_unknown() {
        let icao = Icao::new();
        
        let result = icao.convert("DF=");
        assert_eq!(Err(ConvertError::UnknownCharacter), result);
    }

    #[test]
    fn check_empty() {
        let icao = Icao::new();
        
        let result = icao.convert("");
        let callsign = result.unwrap();
        assert_eq!(callsign, "");
    }
}
