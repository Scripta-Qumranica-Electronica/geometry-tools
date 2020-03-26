<div align="center">

  <h1><code>geometry-tools</code></h1>

  <strong>A WASM (Rust, see <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a></strong>) library for managing various geometric operations on WellKnown and SVG shapes 

</div>

  <h2>Compiling</h2>
  <p>Simply running `wasm-pack build` will generate all the necessary JS/TS files in the `pkg` folder.  The sample website in `www` provides an example of how to use those.</p>

  <h2>Tests</h2>
  <p>Unit tests are written into the individual `.rs` files themselves. They are useful for debugging, since this is a library (and thus has no main function in the binary).</p>
