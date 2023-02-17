# Simulación de series de coordenadas geodésicas con Wasm.

Proyecto en etapa recontra verde.

Para compilarlo simplemente instalar `rust` (con `rustup` funciona), `wasm-pack` y
asegurarse de tener el targuet `wasm32-unknown-unknown` habilitado (por ejemplo
en arch-linux es el paquete rust-wasm, pero si usas rustup se baja solo) y
ejecutar

```
wasm-pack build --target web
```

Listo, Solo que no anda desde archivos locales sino servido, asique para
probarlo hay que usar.

```
python -m http.server
```

o cosas así.
