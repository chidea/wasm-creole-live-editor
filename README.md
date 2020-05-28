## About

[WebASseMbly(WASM)](https://webassembly.org) and [Yew](https://yew.rs) based [WikiCreole](http://wikicreole.org) live editor

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
npm i -g yarn
yarn install
```

### 🛠️ Build

```
yarn build
```

### 🔬 Serve locally

```
yarn run start:dev
```


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
