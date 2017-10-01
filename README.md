# rustrace
A 2D to 1D ray tracer (and visualizer) written in rust

## Examples
![render](doc/example1.png)## Building
(assuming that you already have your `GOPATH` setup correctly)

Just run `go build` in this repo's root, no external libraries are needed

## Usage
```
Usage of ./rtrace:
  -height int
        height of render target
  -width int
        width of render target
  -target string
        render target (use "-" for stdout) (default "-")
```


## Building
(assuming you have the rust toolchain installed)

Just run `cargo build --release` in this repo's root

## Usage
`cargo run --release` OR `./rustrace` (after running the build step).  
There should now be an `output.png` file in the current directory.
