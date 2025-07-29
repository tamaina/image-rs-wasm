import init, { read_and_compress_image } from 'image-rs-wasm'
import type { BrowserImageResizerConfig } from 'image-rs-wasm'

let initialized = false;

onmessage = async (event) => {
    if (!initialized) {
        postMessage({
            status: 'error',
            message: 'Worker not initialized yet',
        });
        return;
    }
    const { file, config } = event.data as {
        file: File,
        config: BrowserImageResizerConfig
    };
    if (!file || !(file instanceof File)) {
        throw new Error("Invalid file input");
    }
    if (!config || typeof config !== 'object') {
        throw new Error("Invalid config input");
    }
    const u8src = await file.arrayBuffer().then(ab => new Uint8Array(ab));
    const u8res = await read_and_compress_image(u8src, config) as Uint8Array<ArrayBuffer>;
    postMessage({
        status: 'success',
        data: u8res,
        config,
    }, [u8res.buffer]);
}

init().then(() => {
    initialized = true;
    postMessage({
        status: 'initialized',
    });
}, (error) => {
    postMessage({
        status: 'error',
        message: 'Initialization failed: ' + error.message,
    });
});
