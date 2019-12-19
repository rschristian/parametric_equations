# Parametric Equation Visualizer [![Build Status](https://travis-ci.org/RyanChristian4427/parametric_equations.svg?branch=master)](https://travis-ci.org/RyanChristian4427/parametric_equations)

This is just a rather simple project that draws randomized parametric equations to a screen, using bright colors and trails.

After starting to write a RESTful API in Rust, I realized I wanted to do a bit more of a complex and logic-based project, as CRUD services are quite monotonous. I had seen some similar visualizations before for other sets of data, so I figured it would be interesting to see visualizations done for randomized parametric equations. 

While Glium is no longer supported by its original author, I chose it as it is still the most complete OpenGL wrapper for Rust, and is quite easy to work with, compared to FFI with C. 

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

```
Rustup
  - Tested against latest stable, beta, and nightly. Look at CI status for more info.
Cargo
```

### Running

To run the app from source: 

```
cargo run --release
```

After the dependencies are downloaded and compiled, the visualizer should be ran. I do recommend running the release build, as there is a large performance increase over the debug build.


### Code Style

All formatting is done with the lovely Rustfmt, which can be ran with:

```
cargo fmt
```

There are a few locations in which fmt is disabled, like the 4d matrices, as 16 lines per matrix is much less clear.

The linter Clippy is also used and often its suggestions are often used, but there are some exceptions. Clippy can be ran with:

```
cargo clippy
```

## Built With

* [Glium](https://github.com/glium/glium) - Safe OpenGL Wrapper used to draw everything on screen
* [Glium Text](https://github.com/tomaka/glium_text) - Easy way to implement text on top of Glium
* [CGMath](https://github.com/rustgd/cgmath) - Linear Algebra and mathematics library used for manipulating the graphics with matrices

## Authors

* **Ryan Christian** - *Entire Project* - [RyanChristian4427](https://github.com/RyanChristian4427)
