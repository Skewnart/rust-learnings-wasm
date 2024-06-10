# Rust learnings WASM (WebAssembly)

## Installations préalables
- Target WASM (32 ou 64 bits)

```
rustup target add wasm32-unknown-unknown
```

- Plugins optionnels :

> A exécuter après la génération, permet de de supprimer les exports inutiles, fonctions, tests, etc.

```
cargo install wasm-gc
```

## Builds et lancements

- Build release / target :

```
cargo build --target wasm32-unknown-unknown --release
wasm-gc ./target/wasm32-unknown-unknown/release/addition_wasm.wasm
```

Attention ici il faudra copier le fichier WASM à l'endroit où le serveur se lance pour le test
```
cp ./target/wasm32-unknown-unknown/release/addition_wasm.wasm ./src/view/
```

Grâce à l'installation préalable, un micro serveur python est lancable *depuis le dossier html*

```
python -m SimpleHTTPServer 8080
```

ou suivant la version

```
python -m http.server 8080
```
