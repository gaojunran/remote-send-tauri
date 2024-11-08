<script setup lang="ts">
import {formatBytes, formatTimeAgo} from "./utils.ts";

const props = defineProps<{
  name?: string,
  size?: number,
  lastTime?: string,
}>()


</script>

<template>
  <Panel class="border-white/50 hover:border-white transition !!px-0">
    <div class="flex items-center justify-start">
      <div class="pi pi-file mr-4"></div>
      <div class="text-sm w-full">
        <div>{{ props.name }}</div>
        <div class="text-xs text-gray-500">{{ formatBytes(props.size) }} / {{ formatTimeAgo(props.lastTime || '') }}</div>
      </div>
    </div>
  </Panel>
</template>

<style scoped>

</style>