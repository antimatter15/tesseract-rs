# tesseract-sys
Rust bindings for [Tesseract](https://github.com/tesseract-ocr/tesseract)

## Building

This links to the C libraries [leptonica](https://github.com/danbloomberg/leptonica) and tesseract.

On Ubuntu and derivatives the additional dependencies can be installed by running:

```bash
sudo apt-get install libleptonica-dev libtesseract-dev clang
```

On Fedora 30 the additional dependencies can be installed by running:

```bash
sudo dnf install leptonica-devel tesseract-devel clang
```

On Termux 2019 (Android, Android on Chromebooks) the additional dependencies can be installed by running:

```bash
pkg install libclang leptonica-dev tesseract-dev
```
