<script setup lang="ts">
import {formatBytes, formatTimeAgo} from "./utils.ts";

const props = defineProps<{
  name?: string,
  size?: number,
  lastTime?: string,
  text?: string
}>()


</script>

<template>
  <Panel class="border-white/50 hover:border-white transition !!px-0">
    <div v-if="props.text">
      <div class="text-sm w-full break-all line-clamp-4">{{ props.text }}</div>
    </div>
    <div v-if="!props.text" class="flex items-center justify-start">
      <div class="pi pi-file mr-4"></div>
      <div class="text-sm w-full break-all">
        <div>{{ props.name }}</div>
        <div class="text-xs text-gray-500">{{ formatBytes(props.size) }} / {{ formatTimeAgo(props.lastTime || '') }}</div>
      </div>
    </div>
  </Panel>
</template>

<style scoped>

</style>