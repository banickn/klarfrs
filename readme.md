# klarfrs
Klarfrs is a parser written in Rust to read klarf files as a python dictionary. Currently it only supports version 1.2.

## Installation

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install klarfrs.

```bash
pip install klarfrs
```

## Usage

```python
import klarfrs

content: dict = klarfrs.parse(filename)
print(content)
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)