<script setup lang="ts">
import { ref } from 'vue'
import ExecWasm from '../workers/exec-wasm?worker';
import type { BrowserImageResizerConfig } from 'image-rs-wasm'

defineProps<{ msg: string }>()

const workerLoaded = ref(false)
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

function revokeUrl() {
  if (url.value) {
    URL.revokeObjectURL(url.value)
    url.value = null
    console.log('Revoked URL:', url.value)
  }
}

async function onClick() {
  revokeUrl()
  if (file.value) {
    const config: BrowserImageResizerConfig = {
      algorithm: 'catmull-rom',
      max_width: maxSize.value,
      max_height: maxSize.value,
      quality: quality.value,
      mime_type: mimeType.value,
    }
    worker.postMessage({
      file: file.value,
      config,
    });
  } else {
    console.warn('No file selected')
  }
}

const worker = new ExecWasm()
worker.onmessage = (event) => {
  if (event.data?.status === 'initialized') {
    workerLoaded.value = true
    console.log('Worker initialized successfully')
  } else if (event.data?.status === 'error') {
    console.error('Worker error:', event.data.error)
  } else if (event.data?.status === 'success') {
    revokeUrl()
    url.value = URL.createObjectURL(new Blob([event.data.data], { type: event.data.config.mime_type }))
    console.log('Image processed successfully:', url.value)
  }
}
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
    <button v-if="workerLoaded" type="button" @click="onClick">CONVERT</button>
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
