<script setup lang="ts">
import { ref } from 'vue'
import init, { read_and_compress_image } from 'image-rs-wasm'
import type { BrowserImageResizerConfig } from 'image-rs-wasm'

defineProps<{ msg: string }>()

const wasmLoaded = ref(false)
const maxSize = ref(1024)
const quality = ref(0.8)
const mimeType = ref('image/avif')
const file = ref<File | null>(null)
const url = ref<string | null>(null)

function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    file.value = target.files[0]
    console.log('Selected file:', file.value)
  }
}

async function onClick() {
  if (url.value) {
    URL.revokeObjectURL(url.value) // Clean up previous URL
    url.value = null // Reset URL
  }
  if (file.value) {
    const config: BrowserImageResizerConfig = {
      algorithm: 'catmull-rom',
      max_width: maxSize.value,
      max_height: maxSize.value,
      quality: quality.value,
      mime_type: 'image/avif',
    }
    const u8src = await file.value.arrayBuffer().then(ab => new Uint8Array(ab));
    const u8res = await read_and_compress_image(u8src, config) as Uint8Array<ArrayBuffer>
    url.value = URL.createObjectURL(new Blob([u8res], { type: config.mime_type }))
  } else {
    console.warn('No file selected')
  }
}

init().then(() => {
  wasmLoaded.value = true
})
</script>

<template>
  <h1>{{ msg }}</h1>
  <p>OffscreenCanvas version (current): <a href="https://github.com/misskey-dev/browser-image-resizer" target="_blank">misskey-dev/browser-image-resizer</a></p>

  <input type="file" accept="image/*" @change="onFileChange" />
  <div class="card">
    <div>
      <label for="maxSize">Max W/H:</label>
      <input type="number" id="maxSize" v-model.number="maxSize" min="1" step="1" />
    </div>
    <div>
      <label for="mimeType">MIME Type:</label>
      <select id="mimeType" v-model="mimeType">
        <option value="image/avif">AVIF</option>
        <option value="image/webp">WEBP</option>
        <option value="image/jpeg">JPEG</option>
        <option value="image/png">PNG</option>
        <option value="image/gif">GIF</option>
      </select>
    </div>
    <button v-if="wasmLoaded" type="button" @click="onClick">CONVERT</button>
  </div>
  <div v-if="url" class="card">
    <img :src="url" alt="Compressed Image" />
    <p><a :href="url" :download="`compressed_image.${mimeType.split('/')[1]}`">Download Compressed Image</a></p>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
