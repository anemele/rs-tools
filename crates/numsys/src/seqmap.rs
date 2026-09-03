use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct SeqMap {
    seq: Vec<char>,
    map: HashMap<char, usize>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SMError {
    #[error("empty sequence")]
    Empty,
    #[error("duplicate character")]
    Duplicate,
}

impl SeqMap {
    pub fn new(char_set: Vec<char>) -> Result<Self, SMError> {
        if char_set.is_empty() {
            return Err(SMError::Empty);
        }

        let mut seq = Vec::new();
        let mut map = HashMap::new();
        for (i, c) in char_set.into_iter().enumerate() {
            if map.contains_key(&c) {
                return Err(SMError::Duplicate);
            }
            map.insert(c, i);
            seq.push(c);
        }
        Ok(Self { seq, map })
    }

    pub fn get_i(&self, c: char) -> Option<usize> {
        self.map.get(&c).cloned()
    }

    pub fn get_c(&self, i: usize) -> Option<char> {
        self.seq.get(i).cloned()
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    #[allow(dead_code)]
    pub fn charset(&self) -> &[char] {
        self.seq.as_slice()
    }
}

impl TryFrom<&str> for SeqMap {
    type Error = SMError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s.chars().collect())
    }
}

impl FromStr for SeqMap {
    type Err = SMError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seqmap() {
        let char_set = "abcde";
        let seqmap = SeqMap::try_from(char_set).unwrap();
        assert_eq!(seqmap.get_i('a'), Some(0));
        assert_eq!(seqmap.get_i('b'), Some(1));
        assert_eq!(seqmap.get_i('c'), Some(2));
        assert_eq!(seqmap.get_i('d'), Some(3));
        assert_eq!(seqmap.get_i('e'), Some(4));
        assert_eq!(seqmap.get_i('f'), None);
        assert_eq!(seqmap.get_c(0), Some('a'));
        assert_eq!(seqmap.get_c(1), Some('b'));
        assert_eq!(seqmap.get_c(2), Some('c'));
        assert_eq!(seqmap.get_c(3), Some('d'));
        assert_eq!(seqmap.get_c(4), Some('e'));
        assert_eq!(seqmap.get_c(5), None);
        assert_eq!(seqmap.len(), 5);
    }

    #[test]
    fn test_seqmap_error() {
        assert_eq!(SeqMap::try_from(""), Err(SMError::Empty));
        assert_eq!(SeqMap::try_from("abca"), Err(SMError::Duplicate));
    }
}
