# RoboRex Learn to Spell

How to run native application:
```
cargo run
```

How to build wasm for running in the browser:
```
cargo +nightly web deploy --target wasm32-unknown-unknown
cp target/deploy/roborex* docs/
python runserver.py
```
