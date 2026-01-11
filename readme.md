# klarfrs
Klarfrs is a parser written in Rust to read klarf files into structured objects. It supports versions 1.2 and 1.8 (via dynamic column handling).

## Installation

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install klarfrs.

```bash
pip install klarfrs
```

## Usage

```python
import klarfrs
from typing import List

filename: str = "klarf.001"

# Parse Header
klarf_header: klarfrs.KlarfData = klarfrs.parse(filename)
print("File Version:", klarf_header.file_version)
print("Lot ID:", klarf_header.lot_id)

# Parse Defects
klarf_defects: List[klarfrs.DefectList] = klarfrs.parse_defects(filename)
for defect in klarf_defects:
    print(f"Defect ID: {defect.defect_id}, X: {defect.xrel}, Y: {defect.yrel}")
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)