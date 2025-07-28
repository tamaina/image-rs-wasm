<script setup lang="ts">
import { ref } from 'vue'
import init, { read_and_compress_image } from 'image-rs-wasm'
import type { BrowserImageResizerConfig } from 'image-rs-wasm'

defineProps<{ msg: string }>()

const wasmLoaded = ref(false)
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
  if (file.value) {
    const config: BrowserImageResizerConfig = {
      algorithm: 'catmull-rom',
      max_width: 1024,
      max_height: 1024,
      quality: 0.8,
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

  <input type="file" accept="image/*" @change="onFileChange" />
  <div class="card">
    <button v-if="wasmLoaded" type="button" @click="onClick">JUST DO IT</button>
  </div>
  <div v-if="url" class="card">
    <img :src="url" alt="Compressed Image" />
    <p><a :href="url" download="compressed_image.png">Download Compressed Image</a></p>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
