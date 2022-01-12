pub struct Cell {
    pub id: u64,
    pub genetic_code: Vec<u8>,
    pub score: f64,
}

impl Cell {
    pub fn new(id: u64, code: Vec<u8>) -> Cell {
        Cell {
            id,
            genetic_code: code,
            score: 0.0,
        }
    }

    pub fn breed(&self, other: &Self) -> Vec<u8> {
        // TODO: Provide different breeding algorithms.
        assert_eq!(self.genetic_code.len(), other.genetic_code.len());
        let pivot: usize = self.genetic_code.len() / 2;
        [&self.genetic_code[..pivot], &other.genetic_code[pivot..]].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn cell_new() {
        let cell = Cell::new(10, Vec::new());
        assert_eq!(cell.id, 10);
        assert_eq!(cell.score, 0.0);
        assert_eq!(cell.genetic_code.len(), 0);
    }

    #[test]
    fn cell_breed() {
        let cell_a = Cell::new(10, vec![1, 2, 3, 4]);
        let cell_b = Cell::new(11, vec![5, 6, 7, 8]);

        let cell_c_code = cell_a.breed(&cell_b);
        assert_eq!(cell_c_code, vec![1, 2, 7, 8]);
    }
}
