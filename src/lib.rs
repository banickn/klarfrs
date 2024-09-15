use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyDict};
use regex::Regex;

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct KlarfData {
    file_version: String, // Klarf file version.
    file_timestamp: String, // Klarf file timestamp.
    inspection_station_id: Vec<String>, // List of inspection station IDs.
    sample_type: String, // Type of sample.
    result_timestamp: String, // Timestamp of the inspection result.
    lot_id: String, // Lot ID.
    sample_size: Vec<u32>, // Sample size in x and y dimensions.
    setup_id: Vec<String>, // List of setup IDs.
    step_id: String, // Step ID.
    wafer_id: String, // Wafer ID.
    slot: u32, // Slot number.
    device_id: String, // Device ID.
    sample_orientation_mark_type: String, // Type of sample orientation mark.
    orientation_mark_location: String, // Location of orientation mark.
    die_pitch: Vec<f64>, // Die pitch in x and y dimensions.
    die_origin: Vec<f64>, // Die origin in x and y coordinates.
    sample_center_location: Vec<f64>, // Sample center location in x and y coordinates.
    orientation_instructions: String, // Orientation instructions.
    coordinates_mirrored: String, // Whether the coordinates are mirrored.
    inspection_orientation: String, // Inspection orientation.
}

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct DefectList {
    defect_id: i64, // Unique identifier for the defect
    xrel: f64, // X coordinate of the defect (relative to the wafer)
    yrel: f64, // Y coordinate of the defect (relative to the wafer)
    xindex: i32, // X index of the defect (related to die location on wafer)
    yindex: i32, // Y index of the defect (related to die location on wafer)
    xsize: f64, // Size of the defect in the X dimension
    ysize: f64, // Size of the defect in the Y dimension
    defect_area: f64, // Area of the defect
    dsize: f64, // Maximum dimension of the defect
    class_number: i32, // Defect classification number
    test: i32, // Test condition or number
    cluster_number: i32, // Cluster number the defect belongs to
    rough_bin_number: i32, // Rough binning classification
    fine_bin_number: i32, // Fine binning classification
    review_sample: i32, // Flag indicating if the defect is a review sample
    adc_size: f64, // Size from ADC measurement (unknown context)
    adc_size_dn_oblique: f64, // ADC size with specific illumination (downward normal oblique)
    adc_size_dw1_oblique: f64, // ADC size with specific illumination (downward 1 oblique)
    adc_size_dw2_oblique: f64, // ADC size with specific illumination (downward 2 oblique)
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
    index1_dn_oblique: i32, // Index 1 with specific illumination (downward normal oblique)
    index1_dw1_oblique: i32, // Index 1 with specific illumination (downward 1 oblique)
    index1_dw2_oblique: i32, // Index 1 with specific illumination (downward 2 oblique)
    index2_dn_oblique: i32, // Index 2 with specific illumination (downward normal oblique)
    index2_dw1_oblique: i32, // Index 2 with specific illumination (downward 1 oblique)
    index2_dw2_oblique: i32, // Index 2 with specific illumination (downward 2 oblique)
    // lpde1: f64,
    // lpde2: f64,
    // lpde3: f64,
    // lpde4: f64,
    // lpm_haze_average_adc_dn_oblique: f64,
    // lpm_haze_average_adc_dw1_oblique: f64,
    // lpm_haze_average_adc_dw2_oblique: f64,
    // lpm_max_adc: f64,
    // lpm_max_adc_dn_oblique: f64,
    // lpm_max_adc_dw1_oblique: f64,
    // lpm_max_adc_dw2_oblique: f64,
    // lpm_max_amplitude: f64,
    // lpm_max_amplitude_dn_oblique: f64,
    // lpm_max_amplitude_dw1_oblique: f64,
    // lpm_max_amplitude_dw2_oblique: f64,
    // lpm_max_amplitude_nppm: f64,
    // lpm_max_amplitude_nppm_dn_oblique: f64,
    // lpm_max_amplitude_nppm_dw1_oblique: f64,
    // lpm_max_amplitude_nppm_dw2_oblique: f64,
    // lpm_max_amplitude_rppm: f64,
    // lpm_max_amplitude_rppm_dn_oblique: f64,
    // lpm_max_amplitude_rppm_dw1_oblique: f64,
    // lpm_max_amplitude_rppm_dw2_oblique: f64,
    // lpm_max_snr: f64,
    // lpm_max_snr_dn_oblique: f64,
    // lpm_max_snr_dw1_oblique: f64,
    // lpm_max_snr_dw2_oblique: f64,
    // lpm_triggered_dn_oblique: i32,
    // lpm_triggered_dw1_oblique: i32,
    // lpm_triggered_dw2_oblique: i32,
    // nppm_size: f64,
    // nppm_size_dn_oblique: f64,
    // nppm_size_dw1_oblique: f64,
    // nppm_size_dw2_oblique: f64,
    // position_r_centroid: f64,
    // position_theta_centroid: f64,
    // rppm_haze_average_dn_oblique: f64,
    // rppm_haze_average_dw1_oblique: f64,
    // rppm_haze_average_dw2_oblique: f64,
    // rppm_size: f64,
    // rppm_size_dn_oblique: f64,
    // rppm_size_dw1_oblique: f64,
    // rppm_size_dw2_oblique: f64,
    // defect_size: f64,
    // size_dn_oblique: f64,
    // size_dn_oblique_to_size_dw1_oblique: f64,
    // size_dn_oblique_to_size_dw2_oblique: f64,
    // size_dw1_oblique: f64,
    // size_dw1_oblique_to_size_dn_oblique: f64,
    // size_dw1_oblique_to_size_dw2_oblique: f64,
    // size_dw2_oblique: f64,
    // size_dw2_oblique_to_size_dn_oblique: f64,
    // size_dw2_oblique_to_size_dw1_oblique: f64,
    // sn_ratio: f64,
    // sn_ratio_dn_oblique: f64,
    // sn_ratio_dw1_oblique: f64,
    // sn_ratio_dw2_oblique: f64,
    // area: f64,
    // aspect_ratio: f64,
    // position_box_r_end: f64,
    // position_box_r_start: f64,
    // position_box_theta_end: f64,
    // position_box_theta_start: f64,
    // length: f64,
    // haze_average: f64,
    // class_code: i32,
    // column_index: i32,
    // enc_energy: f64,
    // index1: i32,
    // index2: i32,
    // lpm_haze_average_adc: f64,
    // lpm_triggered: i32,
    // row_index: i32,
    // rppm_haze_average: f64,
    // adc_size_pcc_oblique: f64,
    // class_code_pcc_oblique: i32,
    // column_index_pcc_oblique: i32,
    // defect_size_neg_adc_pcc_oblique: f64,
    // defect_size_pos_adc_pcc_oblique: f64,
    // defect_size_she_pcc_oblique: f64,
    // enc_energy_pcc_oblique: f64,
    // haze_average_pcc_oblique: f64,
    // index1_pcc_oblique: i32,
    // index2_pcc_oblique: i32,
    // lateral_extent_radial_neg_um_pcc_oblique: f64,
    // lateral_extent_radial_pos_um_pcc_oblique: f64,
    // lateral_extent_radial_um_pcc_oblique: f64,
    // lateral_extent_tangential_neg_um_pcc_oblique: f64,
    // lateral_extent_tangential_pos_um_pcc_oblique: f64,
    // lateral_extent_tangential_um_pcc_oblique: f64,
    // lpm_haze_average_adc_pcc_oblique: f64,
    // lpm_max_adc_pcc_oblique: f64,
    // lpm_max_amplitude_pcc_oblique: f64,
    // lpm_max_amplitude_nppm_pcc_oblique: f64,
    // lpm_max_amplitude_rppm_pcc_oblique: f64,
    // lpm_max_snr_pcc_oblique: f64,
    // lpm_triggered_pcc_oblique: i32,
    // nppm_size_pcc_oblique: f64,
    // pcc_adc_ratio_pcc_oblique: f64,
    // pcc_bckgrnd_intensity_adc_pcc_oblique: f64,
    // pcc_num_clustered_defects_pcc_oblique: i32,
    // ppd_polarity: i32,
    // polarity_pcc_oblique: i32,
    // pos_and_neg_separation_radial_um_pcc_oblique: f64,
    // pos_and_neg_separation_tangential_um_pcc_oblique: f64,
    // rppm_haze_average_pcc_oblique: f64,
    // rppm_size_pcc_oblique: f64,
    // size_pcc_oblique: f64,
    // sn_ratio_pcc_oblique: f64,
    // xneg_pcc_oblique: f64,
    // xpos_pcc_oblique: f64,
    // yneg_pcc_oblique: f64,
    // ypos_pcc_oblique: f64,
    // defect_size_neg_adc: f64,
    // defect_size_pos_adc: f64,
    // defect_size_she: f64,
    // lateral_extent_radial_neg_um: f64,
    // lateral_extent_radial_pos_um: f64,
    // lateral_extent_radial_um: f64,
    // lateral_extent_tangential_neg_um: f64,
    // lateral_extent_tangential_pos_um: f64,
    // lateral_extent_tangential_um: f64,
    // pcc_adc_ratio: f64,
    // pcc_bckgrnd_intensity_adc: f64,
    // pcc_num_clustered_defects: i32,
    // pos_and_neg_separation_radial_um: f64,
    // pos_and_neg_separation_tangential_um: f64,
    // xneg: f64,
    // xpos: f64,
    // yneg: f64,
    // ypos: f64,
    // image_count: i32,
    // image_list: String
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
            // lpde1: 0.0,
            // lpde2: 0.0,
            // lpde3: 0.0,
            // lpde4: 0.0,
            // lpm_haze_average_adc_dn_oblique: 0.0,
            // lpm_haze_average_adc_dw1_oblique: 0.0,
            // lpm_haze_average_adc_dw2_oblique: 0.0,
            // lpm_max_adc: 0.0,
            // lpm_max_adc_dn_oblique: 0.0,
            // lpm_max_adc_dw1_oblique: 0.0,
            // lpm_max_adc_dw2_oblique: 0.0,
            // lpm_max_amplitude: 0.0,
            // lpm_max_amplitude_dn_oblique: 0.0,
            // lpm_max_amplitude_dw1_oblique: 0.0,
            // lpm_max_amplitude_dw2_oblique: 0.0,
            // lpm_max_amplitude_nppm: 0.0,
            // lpm_max_amplitude_nppm_dn_oblique: 0.0,
            // lpm_max_amplitude_nppm_dw1_oblique: 0.0,
            // lpm_max_amplitude_nppm_dw2_oblique: 0.0,
            // lpm_max_amplitude_rppm: 0.0,
            // lpm_max_amplitude_rppm_dn_oblique: 0.0,
            // lpm_max_amplitude_rppm_dw1_oblique: 0.0,
            // lpm_max_amplitude_rppm_dw2_oblique: 0.0,
            // lpm_max_snr: 0.0,
            // lpm_max_snr_dn_oblique: 0.0,
            // lpm_max_snr_dw1_oblique: 0.0,
            // lpm_max_snr_dw2_oblique: 0.0,
            // lpm_triggered_dn_oblique: 0,
            // lpm_triggered_dw1_oblique: 0,
            // lpm_triggered_dw2_oblique: 0,
            // nppm_size: 0.0,
            // nppm_size_dn_oblique: 0.0,
            // nppm_size_dw1_oblique: 0.0,
            // nppm_size_dw2_oblique: 0.0,
            // position_r_centroid: 0.0,
            // position_theta_centroid: 0.0,
            // rppm_haze_average_dn_oblique: 0.0,
            // rppm_haze_average_dw1_oblique: 0.0,
            // rppm_haze_average_dw2_oblique: 0.0,
            // rppm_size: 0.0,
            // rppm_size_dn_oblique: 0.0,
            // rppm_size_dw1_oblique: 0.0,
            // rppm_size_dw2_oblique: 0.0,
            // defect_size: 0.0,
            // size_dn_oblique: 0.0,
            // size_dn_oblique_to_size_dw1_oblique: 0.0,
            // size_dn_oblique_to_size_dw2_oblique: 0.0,
            // size_dw1_oblique: 0.0,
            // size_dw1_oblique_to_size_dn_oblique: 0.0,
            // size_dw1_oblique_to_size_dw2_oblique: 0.0,
            // size_dw2_oblique: 0.0,
            // size_dw2_oblique_to_size_dn_oblique: 0.0,
            // size_dw2_oblique_to_size_dw1_oblique: 0.0,
            // sn_ratio: 0.0,
            // sn_ratio_dn_oblique: 0.0,
            // sn_ratio_dw1_oblique: 0.0,
            // sn_ratio_dw2_oblique: 0.0,
            // area: 0.0,
            // aspect_ratio: 0.0,
            // position_box_r_end: 0.0,
            // position_box_r_start: 0.0,
            // position_box_theta_end: 0.0,
            // position_box_theta_start: 0.0,
            // length: 0.0,
            // haze_average: 0.0,
            // class_code: 0,
            // column_index: 0,
            // enc_energy: 0.0,
            // index1: 0,
            // index2: 0,
            // lpm_haze_average_adc: 0.0,
            // lpm_triggered: 0,
            // row_index: 0,
            // rppm_haze_average: 0.0,
            // adc_size_pcc_oblique: 0.0,
            // class_code_pcc_oblique: 0,
            // column_index_pcc_oblique: 0,
            // defect_size_neg_adc_pcc_oblique: 0.0,
            // defect_size_pos_adc_pcc_oblique: 0.0,
            // defect_size_she_pcc_oblique: 0.0,
            // enc_energy_pcc_oblique: 0.0,
            // haze_average_pcc_oblique: 0.0,
            // index1_pcc_oblique: 0,
            // index2_pcc_oblique: 0,
            // lateral_extent_radial_neg_um_pcc_oblique: 0.0,
            // lateral_extent_radial_pos_um_pcc_oblique: 0.0,
            // lateral_extent_radial_um_pcc_oblique: 0.0,
            // lateral_extent_tangential_neg_um_pcc_oblique: 0.0,
            // lateral_extent_tangential_pos_um_pcc_oblique: 0.0,
            // lateral_extent_tangential_um_pcc_oblique: 0.0,
            // lpm_haze_average_adc_pcc_oblique: 0.0,
            // lpm_max_adc_pcc_oblique: 0.0,
            // lpm_max_amplitude_pcc_oblique: 0.0,
            // lpm_max_amplitude_nppm_pcc_oblique: 0.0,
            // lpm_max_amplitude_rppm_pcc_oblique: 0.0,
            // lpm_max_snr_pcc_oblique: 0.0,
            // lpm_triggered_pcc_oblique: 0,
            // nppm_size_pcc_oblique: 0.0,
            // pcc_adc_ratio_pcc_oblique: 0.0,
            // pcc_bckgrnd_intensity_adc_pcc_oblique: 0.0,
            // pcc_num_clustered_defects_pcc_oblique: 0,
            // ppd_polarity: 0,
            // polarity_pcc_oblique: 0,
            // pos_and_neg_separation_radial_um_pcc_oblique: 0.0,
            // pos_and_neg_separation_tangential_um_pcc_oblique: 0.0,
            // rppm_haze_average_pcc_oblique: 0.0,
            // rppm_size_pcc_oblique: 0.0,
            // size_pcc_oblique: 0.0,
            // sn_ratio_pcc_oblique: 0.0,
            // xneg_pcc_oblique: 0.0,
            // xpos_pcc_oblique: 0.0,
            // yneg_pcc_oblique: 0.0,
            // ypos_pcc_oblique: 0.0,
            // defect_size_neg_adc: 0.0,
            // defect_size_pos_adc: 0.0,
            // defect_size_she: 0.0,
            // lateral_extent_radial_neg_um: 0.0,
            // lateral_extent_radial_pos_um: 0.0,
            // lateral_extent_radial_um: 0.0,
            // lateral_extent_tangential_neg_um: 0.0,
            // lateral_extent_tangential_pos_um: 0.0,
            // lateral_extent_tangential_um: 0.0,
            // pcc_adc_ratio: 0.0,
            // pcc_bckgrnd_intensity_adc: 0.0,
            // pcc_num_clustered_defects: 0,
            // pos_and_neg_separation_radial_um: 0.0,
            // pos_and_neg_separation_tangential_um: 0.0,
            // xneg: 0.0,
            // xpos: 0.0,
            // yneg: 0.0,
            // ypos: 0.0,
            // image_count: 0,
            // image_list: String::new(),
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

    /// Converts the DefectList instances to a Python list.
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

        /// Start to parse when "DefectList" is found.
        if line.starts_with("DefectList") {
            parse_list = true;
            continue;
        }
        /// Parse when "DefectList" is found and the table is not empty.
        if parse_list {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() > 50 {
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

/// Checks if the datetime string is in the correct format.
fn is_datetime_valid(datetime_str: &str) -> bool {
    let pattern = r"^\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$"; // MM-DD-YY HH:MM:SS format
    let re = Regex::new(pattern).unwrap();
    re.is_match(datetime_str)
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
            if is_datetime_valid(&filetimestamp) {
                data.file_timestamp = filetimestamp
            }
            else{
                println!("Invalid FileTimestamp: {}", filetimestamp);
            }
        }
        "InspectionStationID" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.inspection_station_id = parts.iter().map(|s| s.trim_end_matches(';').trim_matches('"').to_string()).collect(); // Convert &str to String
        }
        "SampleType" => data.sample_type = value.trim_end_matches(';').to_string(),
        "ResultTimestamp" => data.result_timestamp = value.trim_end_matches(';').to_string(),
        "LotID" => data.lot_id = value.trim_end_matches(';').trim_matches('"').to_string(),
        "SampleSize" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.sample_size = parts.iter().filter_map(|s| s.trim_end_matches(';').parse().ok()).collect();
        }
        "DeviceID" => data.device_id = value.trim_end_matches(';').trim_matches('"').to_string(),
        "SetupID" => {
            let value = value.trim_end_matches(';');
            let parts: Vec<&str> = value.splitn(2, ' ').collect();
            data.setup_id = parts.iter().map(|s| s.trim_end_matches(';').trim_matches('"').to_string()).collect(); // Convert &str to String
        }
        "StepID" => data.step_id = value.trim_end_matches(';').trim_matches('"').to_string(),
        "SampleOrientationMarkType" => data.sample_orientation_mark_type = value.trim_end_matches(';').to_string(),
        "OrientationMarkLocation" => data.orientation_mark_location = value.trim_end_matches(';').to_string(),
        "DiePitch" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.die_pitch = parts.iter().filter_map(|s| s.trim_end_matches(';').parse().ok()).collect();
        }
        "DieOrigin" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.die_origin = parts.iter().filter_map(|s| s.trim_end_matches(';').parse().ok()).collect();
        }
        "WaferID" => data.wafer_id = value.to_string(),
        "Slot" => data.slot = value.trim_end_matches(';').parse().unwrap_or(0),
        "SampleCenterLocation" => {
            let parts: Vec<&str> = value.split(' ').collect();
            data.sample_center_location = parts.iter().filter_map(|s| s.trim_end_matches(';').parse().ok()).collect();
        }
        "OrientationInstructions" => data.orientation_instructions =  value.trim_end_matches(';').trim_matches('"').trim_end().to_string(),
        "CoordinatesMirrored" => data.coordinates_mirrored = value.trim_end_matches(';').to_string(),
        "InspectionOrientation" => data.inspection_orientation = value.trim_end_matches(';').to_string(),
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