<script setup lang="ts">
import {formatBytes, formatTimeAgo, isText} from "./utils.ts";

const props = defineProps<{
  name: string,
  size?: number,
  lastTime?: string,
  allowDelete?: boolean,
  allowPaste?: boolean,
  text?: string  /* not necessarily from this file */
}>()

const emit = defineEmits<{
  (e: "delete", file: FileDetail): void
}>()


</script>

<template>
  <Panel class="border-white/50 hover:border-white transition !!px-0">
    <div class="flex items-center justify-between">
      <div v-if="isText(props.name)">
        <div class="text-sm w-full break-all line-clamp-4">{{ props.text }}</div>
        <div class="text-xs text-gray-500 mt-1">Pure text</div>
      </div>
      <div v-if="!isText(props.name)" class="flex items-center justify-start">
        <div class="pi pi-file mr-4"></div>
        <div class="text-sm w-full break-all">
          <div>{{ props.name }}</div>
          <div class="text-xs text-gray-500">
            {{ formatBytes(props.size) }}
            {{ props.lastTime ? ' / ' : '' }}
            {{ formatTimeAgo(props.lastTime || '') }}
          </div>
        </div>
      </div>
      <Button v-if="props.allowDelete" class="ml-2 text-red-500 flex-none"
              aria-label="Delete" variant="text" severity="danger"
              @click="() => emit('delete')" icon="pi pi-trash"/>
    </div>

  </Panel>
</template>

<style scoped>

</style>