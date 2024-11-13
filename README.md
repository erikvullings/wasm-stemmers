# WASM Stemmers

A multilingual Porter stemmer for the following languages: Arabic, Danish, Dutch, English, Finnish, French, German, Greek, Hungarian, Italian, Norwegian, Portuguese, Romanian, Russian, Spanish, Swedish, Tamil, and Turkish. 

It is a WASM wrapper around the [rust_stemmers](https://crates.io/crates/rust-stemmers) crate, which implements the algorithms found in the [snowball project](http://snowballstem.org/).

## Development instructions

Assuming you have installed `rust` and [wasm-pack](https://github.com/rustwasm/wasm-pack), you can play with the code in `lib.rs`. 
The `www` folder is based on the Rust and WebAssembly project 

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## Usage

If you are using WebPack v5, make sure you are setting the `experiments: { asyncWebAssembly: true }` property to true. See the `www` folder for an example.

```js
import { LanguageStemmer } from "wasm-stemmers";

const text = "Hello, beautiful world! How are you today - beautiful?";
alert(
  LanguageStemmer.cleanText(text)
    .map((w) => `"${w}"`)
    .join(", ")
);

function testStemmer() {
  let stemmer = new LanguageStemmer("en");
  alert(
    stemmer
      .stemText(text)
      .map((w) => `"${w}"`)
      .join(", ")
  );
}

testStemmer();
```

### 🛠️ Build with `wasm-pack build`

```bash
wasm-pack build
# Optimize for speed.
wasm-opt -O -o pkg/wasm_stemmers_bg.wasm pkg/wasm_stemmers_bg.wasm
```


### 🔬 Test in Headless Browsers with `wasm-pack test`

```bash
wasm-pack test --headless --firefox
```

### 🔬 Test in browser with `npm start`

```bash
cd www
npm i       #  When updating the lib.rs, you need to rerun this to install it in the folder.
npm start   # Open a browser to test your code in `index.js`
```

### 🎁 Publish to NPM with `wasm-pack publish`

```bash
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.

## License

Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
