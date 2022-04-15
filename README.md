# VobSub converter to .srt

## Need -: 

No actually free online service exists for this. Open source projects also use tesseract *but* they use modules for it based on their language which need tesseract build from the ground up and I had a lot of problem due to that.

## Requirements

Tesseract - [Download for Windows](https://github.com/UB-Mannheim/tesseract/wiki), [can be built](https://github.com/tesseract-ocr/tesseract), mostly all Linux distros already have it.

If building from source code - Rust compiler.


## Usage
### Using conf.tmol 

Mode - Two options, single or all. 'Single' converts only one file given in the 'file' option. 'All' converts all available .sub/.idx pairs available in the directory ignoring 'file' option.

File - Path to the single file to be converted **without extension** and both the .sub/.idx files should have the same name, 'mode' should not be 'all'.

Tesseract - Path to the tesseract executable.

### Building

```batch
git pull https://github.com/IrishBumfuzzle/sub.idx-convert-to-srt
cd sub.idx-convert-to-srt
cargo build --release
target/release/sub-to-srt.exe
```

### Running

```batch 
./sub-to-srt.exe
```

## Limitations

1. Only extracts the first available language in the .sub file (usually english)
2. Will error out if any file in the directory which is not an .idx file contains ".idx" in its name when using 'all' mode.
3. Occasional OCR problems so subtitles aren't perfect.
