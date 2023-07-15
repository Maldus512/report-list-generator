use std::path::PathBuf;

#[derive(Default)]
pub struct Model {
    pub hovering_files: Vec<PathBuf>,
    pub dropped_files: Vec<(PathBuf, Report)>,
    pub saved: bool,
}

impl Model {
    pub fn is_hovering(self: &Self) -> bool {
        self.hovering_files.len() > 0
    }

    pub fn valid(self: &Self) -> bool {
        !self.dropped_files.iter().any(|(_, r)| {
            if let Report::Error(_) = r {
                true
            } else {
                false
            }
        })
    }
}

#[derive(Default, Clone, Debug)]
pub enum Report {
    #[default]
    Waiting,
    Valid {
        modello: String,
        matricola: String,
        denominazione: String,
        riferimento: String,
    },
    Error(String),
}
