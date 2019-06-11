
const wasm = import('./file_reader');

wasm
    .then(g => g.echo_static_string())
    .catch(console.error);