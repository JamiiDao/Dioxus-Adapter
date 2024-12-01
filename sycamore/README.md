# Yew Template

#### Building
Make sure to install wasm-pack first. Visit the website [https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/)
```sh
wasm-pack build --dev --target web --out-name wasm --out-dir ./resources/pkg
```

#### Serve files
Install `miniserve` crate or any other http serve that supports serving wasm files as `application/wasm` MIME

##### Using miniserve
On a shell, run the  following commands
```sh
cargo install miniserve
```

```sh
miniserve -p 5500 ./resources --index index.html
```

Now the files are available at port [localhost:5500](http://localhost:5500)
