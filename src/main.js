// number: js <-> rust
const wasm_number = Module.cwrap('wasm_number', 'number', ['number', 'number']);
document.getElementById('number').innerText = wasm_number(3, 4.2);

// string: js <-> rust
const wasm_string_to_rust = Module.cwrap('wasm_string_to_rust', null, ['string']);
const wasm_string_to_js = Module.cwrap('wasm_string_to_js', 'string', []);
wasm_string_to_rust("string from js");
document.getElementById('string').innerText = wasm_string_to_js();

// array: js -> rust
const wasm_array_to_rust = Module.cwrap('wasm_array_to_rust', null, ['number', 'number']);
var len = 5;
var bufsize = len * 4;
var bufptr = Module._malloc(bufsize);
Module._memset(bufptr, 0, bufsize)
var buf = new Float32Array(Module.HEAPF32.buffer, bufptr, len);
for (var i = 0; i < len;i ++){
	buf[i] = 1.1 * i;
}
wasm_array_to_rust(len, buf.byteOffset);
Module._free(bufptr); 

// array: rust -> js
const wasm_array_to_js = Module.cwrap('wasm_array_to_js', null, ['number', 'number']);
var len = 6;
var bufsize = len * 4;
var bufptr = Module._malloc(bufsize);
Module._memset(bufptr, 0, bufsize)
var buf = new Float32Array(Module.HEAPF32.buffer, bufptr, len);
wasm_array_to_js(len, buf.byteOffset);
document.getElementById('array').innerText = buf;
Module._free(bufptr); 
