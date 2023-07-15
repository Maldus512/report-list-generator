use std::{path::PathBuf, str};
use tokio::io::AsyncReadExt;
use tokio::{fs::File, io::AsyncWriteExt};

use native_dialog::FileDialog;

use crate::model::Report;

pub async fn read_report(path: PathBuf) -> Report {
    match read_content(path).await {
        Ok((modello, matricola, denominazione, riferimento)) => Report::Valid {
            modello,
            matricola,
            denominazione,
            riferimento,
        },
        Err(string) => Report::Error(string),
    }
}

pub async fn save_report_list(list: Vec<Report>) -> Result<(), ()> {
    let path = FileDialog::new()
        .set_filename("elenco.csv")
        .show_save_single_file()
        .unwrap();

    match path {
        Some(path) => {
            let mut file = File::create(path).await.map_err(|_| ())?;

            let mut contents :String= format!("Molinari Elettromedicale\n\n\nElenco Prodotti\nModello,Matricola,Denominazione,Riferimento\n");
            for line in list {
                if let Report::Valid {
                    modello,
                    matricola,
                    denominazione,
                    riferimento,
                } = line
                {
                    contents += format!(
                        "{},{},{},{}\n",
                        modello, matricola, denominazione, riferimento
                    )
                    .as_str();
                }
            }

            println!("{}", contents);
            file.write(contents.as_bytes()).await.map_err(|_| ())?;
            Ok(())
        }
        None => Err(()),
    }
}

async fn read_content(path: PathBuf) -> Result<(String, String, String, String), String> {
    let mut file = File::open(path)
        .await
        .map_err(|_| "Impossibile aprire il file".to_string())?;

    let mut contents = vec![];
    file.read_to_end(&mut contents)
        .await
        .map_err(|_| "Impossibile leggere il file".to_string())?;

    let mut lines = str::from_utf8(contents.as_slice())
        .map_err(|_| "Contenuto non valido (codifica)".to_string())?
        .split("\r\n");

    let equipment_number = lines
        .nth(5)
        .and_then(|line6| line6.split(",").nth(7))
        .ok_or("Contenuto non valido (equipment number)".to_string())?;
    let serial_number = lines
        .nth(0)
        .and_then(|line7| line7.split(",").nth(7))
        .ok_or("Contenuto non valido (serial number)".to_string())?;
    let model = lines
        .nth(1)
        .and_then(|line9| line9.split(",").nth(7))
        .ok_or("Contenuto non valido (model)".to_string())?;
    let other = lines
        .nth(1)
        .and_then(|line11| line11.split(",").nth(7))
        .ok_or("Contenuto non valido (other)".to_string())?;

    Ok((
        model.into(),
        serial_number.into(),
        other.into(),
        equipment_number.into(),
    ))
}
