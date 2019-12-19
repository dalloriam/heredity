pub struct Cell {
    pub id: u64,
    pub genetic_code: Vec<u8>,
    pub score: f64
}

impl Cell {
    pub fn new(id: u64, code: Vec<u8>) -> Cell {
        return Cell{id: id, genetic_code: code, score: 0.0};
    }

    pub fn breed(&self, other: &Self) -> Vec<u8> {
        assert_eq!(self.genetic_code.len(), other.genetic_code.len());
        let pivot: usize = self.genetic_code.len() / 2;
        return [&self.genetic_code[..pivot], &other.genetic_code[pivot..]].concat();
    }
}
