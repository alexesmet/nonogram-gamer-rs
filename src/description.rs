use ggez::GameError;
use serde::Deserialize;


#[derive(Deserialize,Debug)]
pub struct LevelDescriptionTemplate {
    pub rows: Vec<Vec<usize>>,
    pub cols: Vec<Vec<usize>>
}

impl LevelDescriptionTemplate {
    pub fn from_file(filepath: &str) -> Result<LevelDescriptionTemplate, GameError> {
        let f = std::fs::File::open(filepath)?;
        let level_description: LevelDescriptionTemplate = serde_yaml::from_reader(&f).expect("Malformed level file");
        Ok(level_description)
    }
}

impl Into<LevelDescription> for LevelDescriptionTemplate {
    fn into(self) -> LevelDescription {
        let Self { rows, cols } = self;
        LevelDescription {
            rows: rows.into_iter().map(|i| i.into_iter().map(|j| (j, false)).collect()).collect(),
            cols: cols.into_iter().map(|i| i.into_iter().map(|j| (j, false)).collect()).collect()
        }
    }
}

pub struct LevelDescription {
    pub rows: Vec<Vec<(usize, bool)>>,
    pub cols: Vec<Vec<(usize, bool)>>
}

impl LevelDescription {
    pub fn row_to_line_description(&self, row_id: usize) -> Vec<usize> {
        self.rows[row_id].iter().map(|(i, _)| *i).collect()
    }
    pub fn col_to_line_description(&self, col_id: usize) -> Vec<usize> {
        self.cols[col_id].iter().map(|(i, _)| *i).collect()
    }
}
