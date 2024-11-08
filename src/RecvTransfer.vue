<script setup lang="ts">

import {formatBytes} from "./utils.ts";

const props = defineProps<{
  size: number;
}>()

import {listen} from "@tauri-apps/api/event";
import {computed, ref} from "vue";
import {Channel} from "@tauri-apps/api/core";

const isOpen = defineModel()

const isStart = ref(false);
const isSuccess = ref(false);
const isFailed = ref(false);

const isWaiting = computed(() => isStart.value && !isSuccess.value && !isFailed.value)

const timeCost = ref(0.0);
const errorMsg = ref('');

const bytesNow = ref(0);
const percentNow = computed(() => (bytesNow.value / props.size) * 100)

listen<number>('download_started', (event) => {
  isStart.value = true;
  isSuccess.value = false;
  isFailed.value = false;
})

listen<number>('download_success', (event) => {
  isSuccess.value = true;
  timeCost.value = event.payload;
})

listen<string>('download_failed', (event) => {
  isFailed.value = true;
  errorMsg.value = event.payload;
})

const channel = new Channel<number>();
channel.onmessage = (message) => {
  bytesNow.value = message;
}


</script>

<template>
  <Dialog v-model:visible="isOpen" modal :style="{width: '75%'}" :closable="false">
    <div class="text-xl mb-8">Downloading...</div>
    <ProgressBar mode="determinate" :value="percentNow" v-show="isStart"></ProgressBar>
    <div class="text-xs text-white/50 mt-2" v-show="isStart">
      {{ formatBytes(bytesNow) }} / {{ formatBytes(props.size) }}
    </div>
    <Message severity="success" size="small" class="text-center mt-2"
             v-if="isSuccess">Download successfully! Time cost: {{ timeCost.toFixed(2) }}s
    </Message>
    <Message severity="error" size="small" class="mt-2 break-all"
             v-if="isFailed">{{ errorMsg }}
    </Message>
    <Button icon="pi pi-times" label="Cancel" @click="isOpen = false" class="mt-8 w-full "
            severity="secondary"></Button>
  </Dialog>
</template>