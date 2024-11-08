<script setup lang="ts">
const isOpen = defineModel()
</script>

<template>
  <Dialog v-model:visible="isOpen" modal :style="{width: '75%'}" :closable="false">
    <div class="text-xl mb-8">Uploading file...</div>
    <ProgressBar :value="100"></ProgressBar>
    <div class="text-xs text-white/50 mt-2">100 KB / 100 KB</div>
    <Button icon="pi pi-times" label="Cancel" @click="isOpen = false" class="mt-8 w-full " severity="secondary"></Button>
  </Dialog>
</template>