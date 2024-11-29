use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod utils;

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct KlarfData {
    file_version: String,                 // Klarf file version.
    file_timestamp: String,               // Klarf file timestamp.
    inspection_station_id: Vec<String>,   // List of inspection station IDs.
    sample_type: String,                  // Type of sample.
    result_timestamp: String,             // Timestamp of the inspection result.
    lot_id: String,                       // Lot ID.
    sample_size: Vec<u32>,                // Sample size in x and y dimensions.
    setup_id: Vec<String>,                // List of setup IDs.
    step_id: String,                      // Step ID.
    wafer_id: String,                     // Wafer ID.
    slot: u32,                            // Slot number.
    device_id: String,                    // Device ID.
    sample_orientation_mark_type: String, // Type of sample orientation mark.
    orientation_mark_location: String,    // Location of orientation mark.
    die_pitch: Vec<f64>,                  // Die pitch in x and y dimensions.
    die_origin: Vec<f64>,                 // Die origin in x and y coordinates.
    sample_center_location: Vec<f64>,     // Sample center location in x and y coordinates.
    orientation_instructions: String,     // Orientation instructions.
    coordinates_mirrored: String,         // Whether the coordinates are mirrored.
    inspection_orientation: String,       // Inspection orientation.
}

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct DefectList {
    defect_id: i64,                // Unique identifier for the defect
    xrel: f64,                     // X coordinate of the defect (relative to the wafer)
    yrel: f64,                     // Y coordinate of the defect (relative to the wafer)
    xindex: i32,                   // X index of the defect (related to die location on wafer)
    yindex: i32,                   // Y index of the defect (related to die location on wafer)
    xsize: f64,                    // Size of the defect in the X dimension
    ysize: f64,                    // Size of the defect in the Y dimension
    defect_area: f64,              // Area of the defect
    dsize: f64,                    // Maximum dimension of the defect
    class_number: i32,             // Defect classification number
    test: i32,                     // Test condition or number
    cluster_number: i32,           // Cluster number the defect belongs to
    rough_bin_number: i32,         // Rough binning classification
    fine_bin_number: i32,          // Fine binning classification
    review_sample: i32,            // Flag indicating if the defect is a review sample
    adc_size: f64,                 // Size from ADC measurement (unknown context)
    adc_size_dn_oblique: f64,      // ADC size with specific illumination (downward normal oblique)
    adc_size_dw1_oblique: f64,     // ADC size with specific illumination (downward 1 oblique)
    adc_size_dw2_oblique: f64,     // ADC size with specific illumination (downward 2 oblique)
    class_code_dn_oblique: i32, // Classification code with specific illumination (downward normal oblique)
    class_code_dw1_oblique: i32, // Classification code with specific illumination (downward 1 oblique)
    class_code_dw2_oblique: i32, // Classification code with specific illumination (downward 2 oblique)
    column_index_dn_oblique: i32, // Column index with specific illumination (downward normal oblique)
    column_index_dw1_oblique: i32, // Column index with specific illumination (downward 1 oblique)
    column_index_dw2_oblique: i32, // Column index with specific illumination (downward 2 oblique)
    enc_energy_dn_oblique: f64, // Encapsulated energy with specific illumination (downward normal oblique)
    enc_energy_dw1_oblique: f64, // Encapsulated energy with specific illumination (downward 1 oblique)
    enc_energy_dw2_oblique: f64, // Encapsulated energy with specific illumination (downward 2 oblique)
    haze_average_dn_oblique: f64, // Haze average with specific illumination (downward normal oblique)
    haze_average_dw1_oblique: f64, // Haze average with specific illumination (downward 1 oblique)
    haze_average_dw2_oblique: f64, // Haze average with specific illumination (downward 2 oblique)
    index1_dn_oblique: i32,       // Index 1 with specific illumination (downward normal oblique)
    index1_dw1_oblique: i32,      // Index 1 with specific illumination (downward 1 oblique)
    index1_dw2_oblique: i32,      // Index 1 with specific illumination (downward 2 oblique)
    index2_dn_oblique: i32,       // Index 2 with specific illumination (downward normal oblique)
    index2_dw1_oblique: i32,      // Index 2 with specific illumination (downward 1 oblique)
    index2_dw2_oblique: i32,      // Index 2 with specific illumination (downward 2 oblique)
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
        }
    }

    /// Converts the KlarfData instance to a Python dictionary.
    fn to_py_dict(&self, py: Python<'_>) -> PyObject {
        let dict: Bound<PyDict> = PyDict::new_bound(py);
        dict.set_item("file_version", self.file_version.clone())
            .unwrap();
        dict.set_item("file_timestamp", self.file_timestamp.clone())
            .unwrap();
        dict.set_item("inspection_station_id", self.inspection_station_id.clone())
            .unwrap();
        dict.set_item("sample_type", self.sample_type.clone())
            .unwrap();
        dict.set_item("result_timestamp", self.result_timestamp.clone())
            .unwrap();
        dict.set_item("lot_id", self.lot_id.clone()).unwrap();
        dict.set_item("sample_size", self.sample_size.clone())
            .unwrap();
        dict.set_item("setup_id", self.setup_id.clone()).unwrap();
        dict.set_item("step_id", self.step_id.clone()).unwrap();
        dict.set_item("wafer_id", self.wafer_id.clone()).unwrap();
        dict.set_item("slot", self.slot).unwrap();
        dict.set_item("device_id", self.device_id.clone()).unwrap();
        dict.set_item(
            "sample_orientation_mark_type",
            self.sample_orientation_mark_type.clone(),
        )
        .unwrap();
        dict.set_item(
            "orientation_mark_location",
            self.orientation_mark_location.clone(),
        )
        .unwrap();
        dict.set_item("die_pitch", self.die_pitch.clone()).unwrap();
        dict.set_item("die_origin", self.die_origin.clone())
            .unwrap();
        dict.set_item(
            "sample_center_location",
            self.sample_center_location.clone(),
        )
        .unwrap();
        dict.set_item(
            "orientation_instructions",
            self.orientation_instructions.clone(),
        )
        .unwrap();
        dict.set_item("coordinates_mirrored", self.coordinates_mirrored.clone())
            .unwrap();
        dict.set_item(
            "inspection_orientation",
            self.inspection_orientation.clone(),
        )
        .unwrap();
        dict.into()
    }
}

/// Parses a Klarf file and returns a KlarfData header information instance.
#[pyfunction]
pub fn parse(path: &str) -> PyResult<PyObject> {
    let klarf_data: KlarfData = parse_internal(path)
        .map_err(|e: io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
    Ok(Python::with_gil(|py: Python| klarf_data.to_py_dict(py)))
}

/// Parses a Klarf file and returns a list of DefectList instances.
#[pyfunction]
pub fn parse_defects(path: &str) -> PyResult<PyObject> {
    let defect_lists: Vec<DefectList> = parse_defect_records(path)
        .map_err(|e: io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    // Converts the DefectList instances to a Python list.
    Python::with_gil(|py| {
        let py_list = PyList::empty_bound(py);
        for defect in defect_lists {
            let defect_dict = PyDict::new_bound(py);
            defect_dict.set_item("defect_id", defect.defect_id)?;
            defect_dict.set_item("xrel", defect.xrel)?;
            defect_dict.set_item("yrel", defect.yrel)?;
            defect_dict.set_item("xindex", defect.xindex)?;
            defect_dict.set_item("yindex", defect.yindex)?;
            defect_dict.set_item("xsize", defect.xsize)?;
            defect_dict.set_item("ysize", defect.ysize)?;
            defect_dict.set_item("defect_area", defect.defect_area)?;
            defect_dict.set_item("dsize", defect.dsize)?;
            defect_dict.set_item("class_number", defect.class_number)?;
            defect_dict.set_item("test", defect.test)?;
            defect_dict.set_item("cluster_number", defect.cluster_number)?;
            defect_dict.set_item("rough_bin_number", defect.rough_bin_number)?;
            defect_dict.set_item("fine_bin_number", defect.fine_bin_number)?;
            defect_dict.set_item("review_sample", defect.review_sample)?;
            defect_dict.set_item("adc_size", defect.adc_size)?;
            defect_dict.set_item("adc_size_dn_oblique", defect.adc_size_dn_oblique)?;
            defect_dict.set_item("adc_size_dw1_oblique", defect.adc_size_dw1_oblique)?;
            defect_dict.set_item("adc_size_dw2_oblique", defect.adc_size_dw2_oblique)?;
            defect_dict.set_item("class_code_dn_oblique", defect.class_code_dn_oblique)?;
            defect_dict.set_item("class_code_dw1_oblique", defect.class_code_dw1_oblique)?;
            defect_dict.set_item("class_code_dw2_oblique", defect.class_code_dw2_oblique)?;
            defect_dict.set_item("column_index_dn_oblique", defect.column_index_dn_oblique)?;
            defect_dict.set_item("column_index_dw1_oblique", defect.column_index_dw1_oblique)?;
            defect_dict.set_item("column_index_dw2_oblique", defect.column_index_dw2_oblique)?;
            defect_dict.set_item("enc_energy_dn_oblique", defect.enc_energy_dn_oblique)?;
            defect_dict.set_item("enc_energy_dw1_oblique", defect.enc_energy_dw1_oblique)?;
            defect_dict.set_item("enc_energy_dw2_oblique", defect.enc_energy_dw2_oblique)?;
            defect_dict.set_item("haze_average_dn_oblique", defect.haze_average_dn_oblique)?;
            defect_dict.set_item("haze_average_dw1_oblique", defect.haze_average_dw1_oblique)?;
            defect_dict.set_item("haze_average_dw2_oblique", defect.haze_average_dw2_oblique)?;
            defect_dict.set_item("index1_dn_oblique", defect.index1_dn_oblique)?;
            defect_dict.set_item("index1_dw1_oblique", defect.index1_dw1_oblique)?;
            defect_dict.set_item("index1_dw2_oblique", defect.index1_dw2_oblique)?;
            defect_dict.set_item("index2_dn_oblique", defect.index2_dn_oblique)?;
            defect_dict.set_item("index2_dw1_oblique", defect.index2_dw1_oblique)?;
            defect_dict.set_item("index2_dw2_oblique", defect.index2_dw2_oblique)?;

            //dict.set_item("defects", defects_py_list)?;

            py_list.append(defect_dict)?;
        }
        Ok(py_list.into())
    })
}

/// Parses a Klarf file and returns a list of DefectList instances.
pub fn parse_defect_records(path: &str) -> io::Result<Vec<DefectList>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut parse_list = false;
    let mut records = Vec::new();
    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
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
            if fields.len() > 37 {
                let record = DefectList {
                    defect_id: fields[0].parse().unwrap(),
                    xrel: fields[1].parse().unwrap(),
                    yrel: fields[2].parse().unwrap_or_default(),
                    xindex: fields[3].parse().unwrap_or_default(),
                    yindex: fields[4].parse().unwrap_or_default(),
                    xsize: fields[5].parse().unwrap_or_default(),
                    ysize: fields[6].parse().unwrap_or_default(),
                    defect_area: fields[7].parse().unwrap_or_default(),
                    dsize: fields[8].parse().unwrap_or_default(),
                    class_number: fields[9].parse().unwrap_or_default(),
                    test: fields[10].parse().unwrap_or_default(),
                    cluster_number: fields[11].parse().unwrap_or_default(),
                    rough_bin_number: fields[12].parse().unwrap_or_default(),
                    fine_bin_number: fields[13].parse().unwrap_or_default(),
                    review_sample: fields[14].parse().unwrap_or_default(),
                    adc_size: fields[15].parse().unwrap_or_default(),
                    adc_size_dn_oblique: fields[16].parse().unwrap_or_default(),
                    adc_size_dw1_oblique: fields[17].parse().unwrap_or_default(),
                    adc_size_dw2_oblique: fields[18].parse().unwrap_or_default(),
                    class_code_dn_oblique: fields[19].parse().unwrap_or_default(),
                    class_code_dw1_oblique: fields[20].parse().unwrap_or_default(),
                    class_code_dw2_oblique: fields[21].parse().unwrap_or_default(),
                    column_index_dn_oblique: fields[22].parse().unwrap_or_default(),
                    column_index_dw1_oblique: fields[23].parse().unwrap_or_default(),
                    column_index_dw2_oblique: fields[24].parse().unwrap_or_default(),
                    enc_energy_dn_oblique: fields[25].parse().unwrap_or_default(),
                    enc_energy_dw1_oblique: fields[26].parse().unwrap_or_default(),
                    enc_energy_dw2_oblique: fields[27].parse().unwrap_or_default(),
                    haze_average_dn_oblique: fields[28].parse().unwrap_or_default(),
                    haze_average_dw1_oblique: fields[29].parse().unwrap_or_default(),
                    haze_average_dw2_oblique: fields[30].parse().unwrap_or_default(),
                    index1_dn_oblique: fields[31].parse().unwrap_or_default(),
                    index1_dw1_oblique: fields[32].parse().unwrap_or_default(),
                    index1_dw2_oblique: fields[33].parse().unwrap_or_default(),
                    index2_dn_oblique: fields[34].parse().unwrap_or_default(),
                    index2_dw1_oblique: fields[35].parse().unwrap_or_default(),
                    index2_dw2_oblique: fields[36].parse().unwrap_or_default(),
                };
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
        let line = line.unwrap();
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
