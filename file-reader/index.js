
const wasm = import('./file_reader');

wasm
    // .then(g => g.echo_static_string())
    .then(g => g.echo_file())
    .catch(console.error);