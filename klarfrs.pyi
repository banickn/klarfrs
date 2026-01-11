from typing import List, Optional

class KlarfData:
    file_version: str
    file_timestamp: str
    inspection_station_id: List[str]
    sample_type: str
    result_timestamp: str
    lot_id: str
    sample_size: List[int]
    setup_id: List[str]
    step_id: str
    wafer_id: str
    slot: int
    device_id: str
    sample_orientation_mark_type: str
    orientation_mark_location: str
    die_pitch: List[float]
    die_origin: List[float]
    sample_center_location: List[float]
    orientation_instructions: str
    coordinates_mirrored: str
    inspection_orientation: str
    defect_record_spec: str

    def __init__(self) -> None: ...

class DefectList:
    defect_id: int
    xrel: float
    yrel: float
    xindex: int
    yindex: int
    xsize: float
    ysize: float
    defect_area: float
    dsize: float
    class_number: int
    test: int
    cluster_number: int
    rough_bin_number: int
    fine_bin_number: int
    review_sample: int
    adc_size: float
    adc_size_dn_oblique: float
    adc_size_dw1_oblique: float
    adc_size_dw2_oblique: float
    class_code_dn_oblique: int
    class_code_dw1_oblique: int
    class_code_dw2_oblique: int
    column_index_dn_oblique: int
    column_index_dw1_oblique: int
    column_index_dw2_oblique: int
    enc_energy_dn_oblique: float
    enc_energy_dw1_oblique: float
    enc_energy_dw2_oblique: float
    haze_average_dn_oblique: float
    haze_average_dw1_oblique: float
    haze_average_dw2_oblique: float
    index1_dn_oblique: int
    index1_dw1_oblique: int
    index1_dw2_oblique: int
    index2_dn_oblique: int
    index2_dw1_oblique: int
    index2_dw2_oblique: int

    def __init__(self) -> None: ...

def parse(path: str) -> KlarfData: ...
def parse_defects(path: str) -> List[DefectList]: ...
