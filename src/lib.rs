use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod utils;

#[pyclass(get_all)]
#[derive(Debug, PartialEq, Clone)]
pub struct KlarfData {
    /// Klarf file version.
    pub file_version: String,
    /// Klarf file timestamp.
    pub file_timestamp: String,
    /// List of inspection station IDs.
    pub inspection_station_id: Vec<String>,
    /// Type of sample.
    pub sample_type: String,
    /// Timestamp of the inspection result.
    pub result_timestamp: String,
    /// Lot ID.
    pub lot_id: String,
    /// Sample size in x and y dimensions.
    pub sample_size: Vec<u32>,
    /// List of setup IDs.
    pub setup_id: Vec<String>,
    /// Step ID.
    pub step_id: String,
    /// Wafer ID.
    pub wafer_id: String,
    /// Slot number.
    pub slot: u32,
    /// Device ID.
    pub device_id: String,
    /// Type of sample orientation mark.
    pub sample_orientation_mark_type: String,
    /// Location of orientation mark.
    pub orientation_mark_location: String,
    /// Die pitch in x and y dimensions.
    pub die_pitch: Vec<f64>,
    /// Die origin in x and y coordinates.
    pub die_origin: Vec<f64>,
    /// Sample center location in x and y coordinates.
    pub sample_center_location: Vec<f64>,
    /// Orientation instructions.
    pub orientation_instructions: String,
    /// Whether the coordinates are mirrored.
    pub coordinates_mirrored: String,
    /// Inspection orientation.
    pub inspection_orientation: String,
    /// Defect record specification (columns).
    pub defect_record_spec: String,
}

#[pyclass(get_all)]
#[derive(Debug, PartialEq, Clone)]
pub struct DefectList {
    /// Unique identifier for the defect
    pub defect_id: i64,
    /// X coordinate of the defect (relative to the wafer)
    pub xrel: f64,
    /// Y coordinate of the defect (relative to the wafer)
    pub yrel: f64,
    /// X index of the defect (related to die location on wafer)
    pub xindex: i32,
    /// Y index of the defect (related to die location on wafer)
    pub yindex: i32,
    /// Size of the defect in the X dimension
    pub xsize: f64,
    /// Size of the defect in the Y dimension
    pub ysize: f64,
    /// Area of the defect
    pub defect_area: f64,
    /// Maximum dimension of the defect
    pub dsize: f64,
    /// Defect classification number
    pub class_number: i32,
    /// Test condition or number
    pub test: i32,
    /// Cluster number the defect belongs to
    pub cluster_number: i32,
    /// Rough binning classification
    pub rough_bin_number: i32,
    /// Fine binning classification
    pub fine_bin_number: i32,
    /// Flag indicating if the defect is a review sample
    pub review_sample: i32,
    /// Size from ADC measurement (unknown context)
    pub adc_size: f64,
    /// ADC size with specific illumination (downward normal oblique)
    pub adc_size_dn_oblique: f64,
    /// ADC size with specific illumination (downward 1 oblique)
    pub adc_size_dw1_oblique: f64,
    /// ADC size with specific illumination (downward 2 oblique)
    pub adc_size_dw2_oblique: f64,
    /// Classification code with specific illumination (downward normal oblique)
    pub class_code_dn_oblique: i32,
    /// Classification code with specific illumination (downward 1 oblique)
    pub class_code_dw1_oblique: i32,
    /// Classification code with specific illumination (downward 2 oblique)
    pub class_code_dw2_oblique: i32,
    /// Column index with specific illumination (downward normal oblique)
    pub column_index_dn_oblique: i32,
    /// Column index with specific illumination (downward 1 oblique)
    pub column_index_dw1_oblique: i32,
    /// Column index with specific illumination (downward 2 oblique)
    pub column_index_dw2_oblique: i32,
    /// Encapsulated energy with specific illumination (downward normal oblique)
    pub enc_energy_dn_oblique: f64,
    /// Encapsulated energy with specific illumination (downward 1 oblique)
    pub enc_energy_dw1_oblique: f64,
    /// Encapsulated energy with specific illumination (downward 2 oblique)
    pub enc_energy_dw2_oblique: f64,
    /// Haze average with specific illumination (downward normal oblique)
    pub haze_average_dn_oblique: f64,
    /// Haze average with specific illumination (downward 1 oblique)
    pub haze_average_dw1_oblique: f64,
    /// Haze average with specific illumination (downward 2 oblique)
    pub haze_average_dw2_oblique: f64,
    /// Index 1 with specific illumination (downward normal oblique)
    pub index1_dn_oblique: i32,
    /// Index 1 with specific illumination (downward 1 oblique)
    pub index1_dw1_oblique: i32,
    /// Index 1 with specific illumination (downward 2 oblique)
    pub index1_dw2_oblique: i32,
    /// Index 2 with specific illumination (downward normal oblique)
    pub index2_dn_oblique: i32,
    /// Index 2 with specific illumination (downward 1 oblique)
    pub index2_dw1_oblique: i32,
    /// Index 2 with specific illumination (downward 2 oblique)
    pub index2_dw2_oblique: i32,
}

#[pymethods]
impl DefectList {
    #[new]
    fn new() -> Self {
        DefectList {
            defect_id: 0,
            xrel: 0.0,
            yrel: 0.0,
            xindex: 0,
            yindex: 0,
            xsize: 0.0,
            ysize: 0.0,
            defect_area: 0.0,
            dsize: 0.0,
            class_number: 0,
            test: 0,
            cluster_number: 0,
            rough_bin_number: 0,
            fine_bin_number: 0,
            review_sample: 0,
            adc_size: 0.0,
            adc_size_dn_oblique: 0.0,
            adc_size_dw1_oblique: 0.0,
            adc_size_dw2_oblique: 0.0,
            class_code_dn_oblique: 0,
            class_code_dw1_oblique: 0,
            class_code_dw2_oblique: 0,
            column_index_dn_oblique: 0,
            column_index_dw1_oblique: 0,
            column_index_dw2_oblique: 0,
            enc_energy_dn_oblique: 0.0,
            enc_energy_dw1_oblique: 0.0,
            enc_energy_dw2_oblique: 0.0,
            haze_average_dn_oblique: 0.0,
            haze_average_dw1_oblique: 0.0,
            haze_average_dw2_oblique: 0.0,
            index1_dn_oblique: 0,
            index1_dw1_oblique: 0,
            index1_dw2_oblique: 0,
            index2_dn_oblique: 0,
            index2_dw1_oblique: 0,
            index2_dw2_oblique: 0,
        }
    }
    fn set_field(&mut self, key: &str, value: &str) {
        match key {
            "DEFECTID" => self.defect_id = value.parse().unwrap_or_default(),
            "XREL" => self.xrel = value.parse().unwrap_or_default(),
            "YREL" => self.yrel = value.parse().unwrap_or_default(),
            "XINDEX" => self.xindex = value.parse().unwrap_or_default(),
            "YINDEX" => self.yindex = value.parse().unwrap_or_default(),
            "XSIZE" => self.xsize = value.parse().unwrap_or_default(),
            "YSIZE" => self.ysize = value.parse().unwrap_or_default(),
            "DEFECTAREA" => self.defect_area = value.parse().unwrap_or_default(),
            "DSIZE" => self.dsize = value.parse().unwrap_or_default(),
            "CLASSNUMBER" => self.class_number = value.parse().unwrap_or_default(),
            "TEST" => self.test = value.parse().unwrap_or_default(),
            "CLUSTERNUMBER" => self.cluster_number = value.parse().unwrap_or_default(),
            "ROUGHBINNUMBER" => self.rough_bin_number = value.parse().unwrap_or_default(),
            "FINEBINNUMBER" => self.fine_bin_number = value.parse().unwrap_or_default(),
            "REVIEWSAMPLE" => self.review_sample = value.parse().unwrap_or_default(),
            "ADCSIZE" => self.adc_size = value.parse().unwrap_or_default(),
            "ADCSIZEDNOBLIQUE" => self.adc_size_dn_oblique = value.parse().unwrap_or_default(),
            "ADCSIZEDW1OBLIQUE" => self.adc_size_dw1_oblique = value.parse().unwrap_or_default(),
            "ADCSIZEDW2OBLIQUE" => self.adc_size_dw2_oblique = value.parse().unwrap_or_default(),
            "CLASSCODEDNOBLIQUE" => self.class_code_dn_oblique = value.parse().unwrap_or_default(),
            "CLASSCODEDW1OBLIQUE" => {
                self.class_code_dw1_oblique = value.parse().unwrap_or_default()
            }
            "CLASSCODEDW2OBLIQUE" => {
                self.class_code_dw2_oblique = value.parse().unwrap_or_default()
            }
            "COLUMNINDEXDNOBLIQUE" => {
                self.column_index_dn_oblique = value.parse().unwrap_or_default()
            }
            "COLUMNINDEXDW1OBLIQUE" => {
                self.column_index_dw1_oblique = value.parse().unwrap_or_default()
            }
            "COLUMNINDEXDW2OBLIQUE" => {
                self.column_index_dw2_oblique = value.parse().unwrap_or_default()
            }
            "ENCENERGYDNOBLIQUE" => self.enc_energy_dn_oblique = value.parse().unwrap_or_default(),
            "ENCENERGYDW1OBLIQUE" => {
                self.enc_energy_dw1_oblique = value.parse().unwrap_or_default()
            }
            "ENCENERGYDW2OBLIQUE" => {
                self.enc_energy_dw2_oblique = value.parse().unwrap_or_default()
            }
            "HAZEAVERAGEDNOBLIQUE" => {
                self.haze_average_dn_oblique = value.parse().unwrap_or_default()
            }
            "HAZEAVERAGEDW1OBLIQUE" => {
                self.haze_average_dw1_oblique = value.parse().unwrap_or_default()
            }
            "HAZEAVERAGEDW2OBLIQUE" => {
                self.haze_average_dw2_oblique = value.parse().unwrap_or_default()
            }
            "INDEX1DNOBLIQUE" => self.index1_dn_oblique = value.parse().unwrap_or_default(),
            "INDEX1DW1OBLIQUE" => self.index1_dw1_oblique = value.parse().unwrap_or_default(),
            "INDEX1DW2OBLIQUE" => self.index1_dw2_oblique = value.parse().unwrap_or_default(),
            "INDEX2DNOBLIQUE" => self.index2_dn_oblique = value.parse().unwrap_or_default(),
            "INDEX2DW1OBLIQUE" => self.index2_dw1_oblique = value.parse().unwrap_or_default(),
            "INDEX2DW2OBLIQUE" => self.index2_dw2_oblique = value.parse().unwrap_or_default(),
            _ => {}
        }
    }
}

#[pymethods]
impl KlarfData {
    /// Creates a new KlarfData instance with default values.
    #[new]
    fn new() -> Self {
        KlarfData {
            file_version: String::new(),
            file_timestamp: String::new(),
            inspection_station_id: Vec::new(),
            sample_type: String::new(),
            result_timestamp: String::new(),
            lot_id: String::new(),
            sample_size: Vec::new(),
            setup_id: Vec::new(),
            step_id: String::new(),
            wafer_id: String::new(),
            slot: 0,
            device_id: String::new(),
            sample_orientation_mark_type: String::new(),
            orientation_mark_location: String::new(),
            die_pitch: Vec::new(),
            die_origin: Vec::new(),
            sample_center_location: Vec::new(),
            orientation_instructions: String::new(),
            coordinates_mirrored: String::new(),
            inspection_orientation: String::new(),
            defect_record_spec: String::new(),
        }
    }
}

/// Parses a Klarf file and returns a KlarfData header information instance.
#[pyfunction]
pub fn parse(path: &str) -> PyResult<KlarfData> {
    parse_internal(path)
        .map_err(|e: io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}

/// Parses a Klarf file and returns a list of DefectList instances.
#[pyfunction]
pub fn parse_defects(path: &str) -> PyResult<Vec<DefectList>> {
    parse_defect_records(path)
        .map_err(|e: io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}

/// Parses a Klarf file and returns a list of DefectList instances.
pub fn parse_defect_records(path: &str) -> io::Result<Vec<DefectList>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut parse_list = false;
    let mut records = Vec::new();

    // Default column order based on legacy implementation
    let mut column_specs: Vec<String> = vec![
        "DEFECTID",
        "XREL",
        "YREL",
        "XINDEX",
        "YINDEX",
        "XSIZE",
        "YSIZE",
        "DEFECTAREA",
        "DSIZE",
        "CLASSNUMBER",
        "TEST",
        "CLUSTERNUMBER",
        "ROUGHBINNUMBER",
        "FINEBINNUMBER",
        "REVIEWSAMPLE",
        "ADCSIZE",
        "ADCSIZEDNOBLIQUE",
        "ADCSIZEDW1OBLIQUE",
        "ADCSIZEDW2OBLIQUE",
        "CLASSCODEDNOBLIQUE",
        "CLASSCODEDW1OBLIQUE",
        "CLASSCODEDW2OBLIQUE",
        "COLUMNINDEXDNOBLIQUE",
        "COLUMNINDEXDW1OBLIQUE",
        "COLUMNINDEXDW2OBLIQUE",
        "ENCENERGYDNOBLIQUE",
        "ENCENERGYDW1OBLIQUE",
        "ENCENERGYDW2OBLIQUE",
        "HAZEAVERAGEDNOBLIQUE",
        "HAZEAVERAGEDW1OBLIQUE",
        "HAZEAVERAGEDW2OBLIQUE",
        "INDEX1DNOBLIQUE",
        "INDEX1DW1OBLIQUE",
        "INDEX1DW2OBLIQUE",
        "INDEX2DNOBLIQUE",
        "INDEX2DW1OBLIQUE",
        "INDEX2DW2OBLIQUE",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("DefectRecordSpec") {
            let _parts: Vec<&str> = line.split_whitespace().collect();
            // Expected format: DefectRecordSpec "COL1 COL2 ...";
            // Or: DefectRecordSpec COL1 COL2 ...;
            // Let's handle the quoted string content if present
            let content = line
                .trim_start_matches("DefectRecordSpec")
                .trim()
                .trim_end_matches(';')
                .trim_matches('"');

            column_specs = content.split_whitespace().map(String::from).collect();
            continue;
        }

        // "DefectList" happens after the column description and before the table starts
        if line.starts_with("DefectList") {
            parse_list = true;
            continue;
        }
        // Parse when "DefectList" is found and the table is not empty.
        if parse_list {
            let fields: Vec<&str> = line.split_whitespace().collect();
            // Basic validation: ensure we have columns to map to
            if !fields.is_empty() {
                let mut record = DefectList::new();
                for (i, value) in fields.iter().enumerate() {
                    if i < column_specs.len() {
                        record.set_field(&column_specs[i], value);
                    }
                }
                records.push(record);
            }
        }
    }

    Ok(records)
}

/// Parses a line of the Klarf file into key and values.
fn parse_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() >= 2 {
        let key = parts[0];
        let value = parts[1..].join(" ");
        Some((key.to_string(), value))
    } else {
        None
    }
}

/// Receives klarfdata and matches it with the key and value from the line.
fn parse_data(data: &mut KlarfData, key: String, value: String) {
    match key.as_str() {
        "FileVersion" => data.file_version = value.trim_end_matches(';').to_string(),
        "FileTimestamp" => {
            let filetimestamp = value.trim_end_matches(';').to_string();
            if utils::is_datetime_valid(&filetimestamp) {
                data.file_timestamp = filetimestamp
            } else {
                println!("Invalid FileTimestamp: {}", filetimestamp);
            }
        }
        "InspectionStationID" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.inspection_station_id = parts
                .iter()
                .map(|s| s.trim_end_matches(';').trim_matches('"').to_string())
                .collect(); // Convert &str to String
        }
        "SampleType" => data.sample_type = value.trim_end_matches(';').to_string(),
        "ResultTimestamp" => data.result_timestamp = value.trim_end_matches(';').to_string(),
        "LotID" => data.lot_id = value.trim_end_matches(';').trim_matches('"').to_string(),
        "SampleSize" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.sample_size = parts
                .iter()
                .filter_map(|s| s.trim_end_matches(';').parse().ok())
                .collect();
        }
        "DeviceID" => data.device_id = value.trim_end_matches(';').trim_matches('"').to_string(),
        "SetupID" => {
            let value = value.trim_end_matches(';');
            let parts: Vec<&str> = value.splitn(2, ' ').collect();
            data.setup_id = parts
                .iter()
                .map(|s| s.trim_end_matches(';').trim_matches('"').to_string())
                .collect(); // Convert &str to String
        }
        "StepID" => data.step_id = value.trim_end_matches(';').trim_matches('"').to_string(),
        "SampleOrientationMarkType" => {
            data.sample_orientation_mark_type = value.trim_end_matches(';').to_string()
        }
        "OrientationMarkLocation" => {
            data.orientation_mark_location = value.trim_end_matches(';').to_string()
        }
        "DiePitch" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.die_pitch = parts
                .iter()
                .filter_map(|s| s.trim_end_matches(';').parse().ok())
                .collect();
        }
        "DieOrigin" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.die_origin = parts
                .iter()
                .filter_map(|s| s.trim_end_matches(';').parse().ok())
                .collect();
        }
        "WaferID" => data.wafer_id = value.to_string(),
        "Slot" => data.slot = value.trim_end_matches(';').parse().unwrap_or(0),
        "SampleCenterLocation" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.sample_center_location = parts
                .iter()
                .filter_map(|s| s.trim_end_matches(';').parse().ok())
                .collect();
        }
        "OrientationInstructions" => {
            data.orientation_instructions = value
                .trim_end_matches(';')
                .trim_matches('"')
                .trim_end()
                .to_string()
        }
        "CoordinatesMirrored" => {
            data.coordinates_mirrored = value.trim_end_matches(';').to_string()
        }
        "InspectionOrientation" => {
            data.inspection_orientation = value.trim_end_matches(';').to_string()
        }
        "DefectRecordSpec" => data.defect_record_spec = value.trim_end_matches(';').to_string(),
        _ => {}
    }
}

/// Parses a Klarf file and returns a KlarfData instance.
pub fn parse_internal(path: &str) -> io::Result<KlarfData> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut klarf_data = KlarfData::new();

    for line in reader.lines() {
        let line = line?;
        // No need to parse the Defect List for the header information
        if line.starts_with("DefectList") {
            break;
        }
        if let Some((key, value)) = parse_line(&line) {
            parse_data(&mut klarf_data, key, value);
        }
    }
    Ok(klarf_data)
}

#[pymodule]
/// The module entry point for the klarfrs Python module.
fn klarfrs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_defects, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn test_is_datetime_valid() {
        assert!(utils::is_datetime_valid("08-12-24 23:41:25"));
        assert!(!utils::is_datetime_valid("2024-01-15 10:30:00")); // Incorrect format
        assert!(!utils::is_datetime_valid("08-12-24")); // Missing time part
    }

    #[test]
    fn test_parse_line_valid() {
        let line = "FileVersion 1 2";
        let result = parse_line(line);
        assert_eq!(result, Some(("FileVersion".to_string(), "1 2".to_string())));
    }

    #[test]
    fn test_parse_line_invalid() {
        let line = "ThisLineHasNoSpace";
        let result = parse_line(line);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_defect_records_empty() {
        let temp_file = NamedTempFile::new().unwrap();
        // Don't write DefectList records to the file

        let file_path = temp_file.path().to_str().unwrap();
        let result = parse_defect_records(file_path).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_defect_records_with_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        {
            let file = temp_file.as_file_mut();
            writeln!(file, "DefectList").unwrap();
            writeln!(file, "1 12041.000 149816.184 0 0 0.000 0.000 0.000000 0.020 0 6 0 0 0 0 67.084091 13.767257 67.084091 -10.763492 0 0 0 1018.000000 1026.000000 1018.000000 1.000000 0.952043 0.664093 0.228342 0.018191 0.175617 0 2662 0 0 0 0 0.000000 0.000000 0.000000 0.000000 606.512512 606.510620 606.510620 29.489401 29.487518 29.489401 29.489401 0.062226 0.062225 0.062226 0.062226 0.001681 0.000000 0.000000 0.000000 0.003093 0.000000 0.000000 0.000000 2.241958 2.241876 2.241958 2.241958 0.000000 0.000000 0.000000 0.001841 0.000778 0.001841 0.000000 137.959122 180.076340 0.219439 0.019687 0.187926 0.003967 0.000897 0.003967 0.000000 0.020332 0.027163 1.335966 17.000000 0.020332 0.748522 17.000000 0.000000 16.000000 16.000000 31.520321 2.896732 31.520321 -2.680547 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0.000000 0.000000 0 0 0.000000 0.000000 0.000000 0.000000 0.000000 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0").unwrap();
            writeln!(file, "2 146023.921 112165.243 0 0 0.000 0.000 0.000000 0.020 0 7 0 0 0 0 67.084091 0.000000 0.000000 0.000000 0 0 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0 0 0 0 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 29.489401 0.000000 0.000000 0.000000 0.062226 0.000000 0.000000 0.000000 0.001681 0.000000 0.000000 0.000000 0.003093 0.000000 0.000000 0.000000 2.241958 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.001841 0.000000 0.000000 0.000000 137.959122 180.076340 0.000000 0.000000 0.000000 0.003967 0.000000 0.000000 0.000000 0.020332 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 31.520321 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.018191 0 1026.000000 0.952043 2662 0 606.510620 0.000000 3009.000000 0.019687 0.000000 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0 0").unwrap();
        }
        let file_path = temp_file.path().to_str().unwrap();
        let result = parse_defect_records(file_path).unwrap();

        assert_eq!(result.len(), 2); // Check if two defect record are parsed

        let defectid_1 = &result[0];
        assert_eq!(defectid_1.defect_id, 1);
        assert_eq!(defectid_1.xrel, 12041.000);
        assert_eq!(defectid_1.yrel, 149816.184);
        let defectid_2 = &result[1];
        assert_eq!(defectid_2.defect_id, 2);
        assert_eq!(defectid_2.xrel, 146023.921);
        assert_eq!(defectid_2.yrel, 112165.243);
    }
}
