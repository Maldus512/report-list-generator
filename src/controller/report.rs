use rust_xlsxwriter::*;
use std::{path::PathBuf, str};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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

pub async fn save_report_list(list: Vec<Report>) -> Result<(), XlsxError> {
    let path = FileDialog::new()
        .set_filename("elenco.xlsx")
        .show_save_single_file()
        .unwrap();

    match path {
        Some(path) => {
            /*
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
            */
            // Create a new Excel file object.
            let mut workbook = Workbook::new();

            // Create some formats to use in the worksheet.
            let bold_format = Format::new().set_bold();
            let date_format = Format::new().set_num_format("dd/mm/yyyy").set_align(FormatAlign::Center);
            let title_format = Format::new().set_background_color(Color::RGB(0x70AD47)).set_border(FormatBorder::Hair);
            let even_format = Format::new().set_background_color(Color::RGB(0xE2EFD9)).set_border(FormatBorder::Hair);
            let odd_format = Format::new().set_background_color(Color::RGB(0xD9E2F3)).set_border(FormatBorder::Hair);
            let center_format = Format::new().set_align(FormatAlign::Center);

            // Add a worksheet to the workbook.
            let worksheet = workbook.add_worksheet();

            worksheet.set_column_width(0, 30)?;
            worksheet.set_column_width(1, 25)?;
            worksheet.set_column_width(2, 25)?;
            worksheet.set_column_width(3, 25)?;

            // Write a string without formatting.
            worksheet.write_with_format(0, 0, "MOLINARI ELETTROMEDICALI Snc", &bold_format)?;

            worksheet.write_with_format(2, 0, "MEDGROUP CASTELLARANO", &center_format)?;
            worksheet.write(3, 0, "Elenco Prodotti")?;

            worksheet.write_with_format(4, 0, "Modello", &title_format)?;
            worksheet.write_with_format(4, 1, "Matricola", &title_format)?;
            worksheet.write_with_format(4, 2, "Denominazione", &title_format)?;
            worksheet.write_with_format(4, 3, "Riferimento", &title_format)?;

            // Write a date.
            let start = std::time::SystemTime::now();
            let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH).unwrap();
            let date = ExcelDateTime::from_timestamp(since_the_epoch.as_secs() as i64)?;
            worksheet.write_with_format(2, 2, &date, &date_format)?;

            for (i, line) in list.iter().enumerate() {
                if let Report::Valid {
                    modello,
                    matricola,
                    denominazione,
                    riferimento,
                } = line
                {
                    let format = if i % 2 == 0 {
                        &even_format
                    } else {
                        &odd_format
                    };

                    worksheet.write_with_format(5 + i as u32, 0, modello, format)?;
                    worksheet.write_with_format(5 + i as u32, 1, matricola, format)?;
                    worksheet.write_with_format(5 + i as u32, 2, denominazione, format)?;
                    worksheet.write_with_format(5 + i as u32, 3, riferimento, format)?;
                }
            }

            // Save the file to disk.
            println!("Saving workbook to {}", path.display());
            workbook.save(path)
        }
        None => Err(XlsxError::ParameterError("Invalid path".into())),
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
