<script setup lang="ts">
import {formatBytes, formatTimeAgo, isText, isZipped} from "./utils.ts";
import {useClipboard} from "@vueuse/core";

const props = defineProps<{
  name: string,
  size?: number,
  lastTime?: string,
  allowDelete?: boolean,
  allowCopyText?: boolean,
  text?: string  /* not necessarily from this file */
}>()

const emit = defineEmits(['delete'])

const {copy, copied} = useClipboard({source: props.text})

</script>

<template>
  <Panel pt:header:class="h-0 p-0" pt:content:class="py-2" class="border-white/50 hover:border-white transition !!px-0">
      <div v-if="isText(props.name)" class="flex items-center justify-between">
        <div class="pi pi-align-justify mr-4 flex-none"></div>
        <div class="text-sm w-full break-all line-clamp-4 grow">
          {{ (props.text === undefined) ?  // when receiving a text and not known its content
            '[ Pure text ]'
            : props.text }}
        </div>
        <Button v-if="props.allowDelete" class="ml-2 text-red-500 flex-none"
                aria-label="Delete" variant="text" severity="danger"
                @click="() => emit('delete')" icon="pi pi-trash"/>
        <Button v-if="props.allowCopyText && isText(props.name)" class="ml-2 text-white/50 flex-none"
                aria-label="Paste" variant="text" severity="info"
                @click="copy()" :icon="copied? 'pi pi-check' : 'pi pi-copy'"/>
      </div>
      <div v-if="!isText(props.name)" class="flex items-center justify-between">
        <div class="pi pi-file mr-4 flex-none" v-if="!isZipped(props.name)"></div>
        <div class="pi pi-truck mr-4 flex-none" v-if="isZipped(props.name)"></div>
        <div class="text-sm flex-1 break-all">
          <div class="line-clamp-2">{{ isZipped(props.name) ? '[ Multiple files ]' : props.name }}</div>
          <div class="text-xs text-gray-500">
            {{ formatBytes(props.size) }}
            {{ props.lastTime ? ' / ' : '' }}
            {{ formatTimeAgo(props.lastTime || '') }}
          </div>
        </div>
        <Button v-if="props.allowDelete" class="ml-2 text-red-500 flex-none"
                aria-label="Delete" variant="text" severity="danger"
                @click="emit('delete')" icon="pi pi-trash"/>
      </div>

  </Panel>
</template>

<style scoped>

</style>