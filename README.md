# img2ico

Welcome to img2ico, a windows icon making utility. 

## Standalone app usage

Add `img2ico.exe` to your `PATH` and execute it with the image you wish to
convert as the first and only argument:

``` img2ico <image path>.png ```

This will create a .ico file in the same directory with the same name as the
source file.

## Usage as a library in your Rust application

Add the following line to your `cargo.toml` file
```
img2ico = { git = "https://github.com/outphase/img2ico"}
```
