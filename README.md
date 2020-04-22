<div align="center">

  <h1><code>geometry-tools</code></h1>

  <strong>A WASM (Rust, see <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a></strong>) library for managing various geometric operations on WellKnown and SVG shapes 

</div>

  <h2>Compiling</h2>
  <p>Simply running `wasm-pack build` will generate all the necessary JS/TS files in the `pkg` folder.  The sample website in `www` provides an example of how to use those.</p>

  <h2>Tests</h2>
  <p>Some unit tests can be found in the individual `.rs` files themselves.</p>

  <h3>Dependencies</h3>
  <p>Among the many fine dependencies used in this library, the major packages are <a href="https://github.com/georust/geo">geo</a>, <a href="https://github.com/georust/wkt">wkt</a>, and <a href="https://github.com/21re/rust-geo-booleanop">rust-geo-booleanop</a>.</p>
