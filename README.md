## About

[WikiCreole](http://wikicreole.org) editor with live preview based on [WebASseMbly(WASM)](https://webassembly.org) over [Sycamore](https://sycamore-rs.netlify.app)

### Features

- WASM-fast live HTML preview
- Cross-platform installable Progressive Web Application(PWA)
 - Automatic updates on every startups
- Autosave
- Hackable Javascript links (raw <a> tag)

### Web preview

[On Github Pages](https://chidea.github.io)

### Dependencies

When building for the first time, ensure to install dependencies first.

```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

### 🔬 Serve locally for test (on http://localhost:8080)

```
trunk serve
```

### 🛠️ Build for production

```
trunk build --release
```


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.