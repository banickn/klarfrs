use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct KlarfData {
    pub file_version: String,
    pub file_timestamp: String,
    pub inspection_station_id: Vec<String>,
    pub sample_type: String,
    pub result_timestamp: String,
    pub lot_id: String,
    pub sample_size: Vec<u32>,
    pub setup_id: Vec<String>,
    pub step_id: String,
    pub wafer_id: String,
    pub slot: u32,
    pub device_id: String,
    pub sample_orientation_mark_type: String,
    pub orientation_mark_location: String,
    pub die_pitch: Vec<f64>,
    pub die_origin: Vec<f64>,
    pub sample_center_location: Vec<f64>,
    pub orientation_instructions: String,
    pub coordinates_mirrored: String,
    pub inspection_orientation: String,
}

#[pymethods]
impl KlarfData {
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
    fn to_py_dict(&self, py: Python<'_>) -> PyObject {
        let dict: Bound<PyDict> = PyDict::new_bound(py);
        dict.set_item("file_version", self.file_version.clone()).unwrap();
        dict.set_item("file_timestamp", self.file_timestamp.clone()).unwrap();
        dict.set_item("inspection_station_id", self.inspection_station_id.clone()).unwrap();
        dict.set_item("sample_type", self.sample_type.clone()).unwrap();
        dict.set_item("result_timestamp", self.result_timestamp.clone()).unwrap();
        dict.set_item("lot_id", self.lot_id.clone()).unwrap();
        dict.set_item("sample_size", self.sample_size.clone()).unwrap();
        dict.set_item("setup_id", self.setup_id.clone()).unwrap();
        dict.set_item("step_id", self.step_id.clone()).unwrap();
        dict.set_item("wafer_id", self.wafer_id.clone()).unwrap();
        dict.set_item("slot", self.slot).unwrap();
        dict.set_item("device_id", self.device_id.clone()).unwrap();
        dict.set_item("sample_orientation_mark_type", self.sample_orientation_mark_type.clone()).unwrap();
        dict.set_item("orientation_mark_location", self.orientation_mark_location.clone()).unwrap();
        dict.set_item("die_pitch", self.die_pitch.clone()).unwrap();
        dict.set_item("die_origin", self.die_origin.clone()).unwrap();
        dict.set_item("sample_center_location", self.sample_center_location.clone()).unwrap();
        dict.set_item("orientation_instructions", self.orientation_instructions.clone()).unwrap();
        dict.set_item("coordinates_mirrored", self.coordinates_mirrored.clone()).unwrap();
        dict.set_item("inspection_orientation", self.inspection_orientation.clone()).unwrap();
        dict.into()
    }
}

#[pyfunction]
pub fn parse(path: &str) -> PyResult<PyObject> {
    let klarf_data: KlarfData = parse_internal(path)
        .map_err(|e: io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
    Ok(Python::with_gil(|py: Python| klarf_data.to_py_dict(py)))
}

fn parse_internal(path: &str) -> io::Result<KlarfData> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut klarf_data = KlarfData::new();

    let mut parsers: HashMap<&str, Box<dyn Fn(&mut KlarfData, String)>> = HashMap::new();
    parsers.insert("FileVersion", Box::new(|data, line| {
        data.file_version = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("FileTimestamp", Box::new(|data, line| {
        data.file_timestamp = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("InspectionStationID", Box::new(|data, line| {
        data.inspection_station_id = line.trim_end_matches(';')
            .split('"')
            .filter(|s| !s.is_empty() && s.trim() != ";" && s.trim() != "")
            .map(|s| s.trim().to_string())
            .collect();
    }));
    parsers.insert("SampleType", Box::new(|data, line| {
        data.sample_type = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("ResultTimestamp", Box::new(|data, line| {
        data.result_timestamp = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("LotID", Box::new(|data, line| {
        data.lot_id = line.trim_end_matches(';').trim_matches('"').to_string();
    }));
    parsers.insert("SampleSize", Box::new(|data, line| {
        data.sample_size = line.trim_end_matches(';')
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }));
    parsers.insert("SetupID", Box::new(|data, line| {
        data.setup_id = line.trim_end_matches(';')
            .split('"')
            .filter(|s| !s.is_empty() && s.trim() != ";")
            .map(|s| s.trim().to_string())
            .collect();
    }));
    parsers.insert("StepID", Box::new(|data, line| {
        data.step_id = line.trim_end_matches(';').trim_matches('"').to_string();
    }));
    parsers.insert("WaferID", Box::new(|data, line| {
        data.wafer_id = line.trim_end_matches(';').trim_matches('"').to_string();
    }));
    parsers.insert("Slot", Box::new(|data, line| {
        data.slot = line.trim_end_matches(';').parse().unwrap_or(0);
    }));
    parsers.insert("DeviceID", Box::new(|data, line| {
        data.device_id = line.trim_end_matches(';').trim_matches('"').to_string();
    }));
    parsers.insert("SampleOrientationMarkType", Box::new(|data, line| {
        data.sample_orientation_mark_type = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("OrientationMarkLocation", Box::new(|data, line| {
        data.orientation_mark_location = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("DiePitch", Box::new(|data, line| {
        data.die_pitch = line.trim_end_matches(';').split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }));
    parsers.insert("DieOrigin", Box::new(|data, line| {
        data.die_origin = line.trim_end_matches(';').split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }));
    parsers.insert("SampleCenterLocation", Box::new(|data, line| {
        data.sample_center_location = line.trim_end_matches(';').split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }));
    parsers.insert("OrientationInstructions", Box::new(|data, line| {
        data.orientation_instructions = line.trim_end_matches(';').trim_matches('"').to_string();
    }));
    parsers.insert("CoordinatesMirrored", Box::new(|data, line| {
        data.coordinates_mirrored = line.trim_end_matches(';').to_string();
    }));
    parsers.insert("InspectionOrientation", Box::new(|data, line| {
        data.inspection_orientation = line.trim_end_matches(';').to_string();
    }));

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if let Some(key) = parts.get(0) {
            if let Some(parser) = parsers.get(key.trim_end_matches(':')) {
                let value = parts[1..].join(" ");
                parser(&mut klarf_data, value);
            }
        }
    }

    Ok(klarf_data)
}

#[pymodule]
fn klarfrs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}