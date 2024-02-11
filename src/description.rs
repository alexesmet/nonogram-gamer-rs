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
            rows: rows.into_iter()
                .map(|i| LineDescription {
                    parts: i.into_iter()
                        .map(|j| LineDescriptionPart { elements_count: j, is_completed: false })
                        .collect()
                })
                .collect(),
            cols: cols.into_iter()
                .map(|i| LineDescription {
                    parts: i.into_iter()
                        .map(|j| LineDescriptionPart { elements_count: j, is_completed: false })
                        .collect()
                })
                .collect()
        }
    }
}

pub struct LineDescriptionPart {
    pub elements_count: usize,
    pub is_completed: bool
}
pub struct LineDescription {
    pub parts: Vec<LineDescriptionPart>
}
pub struct LevelDescription {
    pub rows: Vec<LineDescription>,
    pub cols: Vec<LineDescription>
}

impl LevelDescription {
    pub fn row_to_line_description(&self, row_id: usize) -> Vec<usize> {
        self.rows[row_id].parts.iter().map(|(x)| x.elements_count).collect()
    }
    pub fn col_to_line_description(&self, col_id: usize) -> Vec<usize> {
        self.cols[col_id].parts.iter().map(|(x)| x.elements_count).collect()
    }

    pub fn is_done(&self) -> bool {
        self.rows.iter().all(|x| x.parts.iter().all(|y| y.is_completed )) && self.cols.iter().all(|x| x.parts.iter().all(|y| y.is_completed))
    }
}
