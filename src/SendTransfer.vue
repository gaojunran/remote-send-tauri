<script setup lang="ts">
import {listen} from "@tauri-apps/api/event";
import {computed, ref} from "vue";

const isOpen = defineModel()

const isStart = ref(false);
const isSuccess = ref(false);
const isFailed = ref(false);

const isWaiting = computed(() => isStart.value && !isSuccess.value && !isFailed.value)

const timeCost = ref(0.0);
const errorMsg = ref('');

listen<number>('upload_started', (event) => {
  isStart.value = true;
  isSuccess.value = false;
  isFailed.value = false;
})

listen<number>('upload_success', (event) => {
  isSuccess.value = true;
  timeCost.value = event.payload;
})

listen<string>('upload_failed', (event) => {
  isFailed.value = true;
  errorMsg.value = event.payload;
})

</script>

<template>
  <Dialog v-model:visible="isOpen" modal :style="{width: '75%'}" :closable="false">
    <div class="text-xl mb-8">Uploading...</div>
    <ProgressBar mode="indeterminate" v-show="isWaiting"></ProgressBar>
    <div class="text-xs text-white/50 mt-2" v-show="isWaiting">Please wait for a while...</div>
    <Message severity="success" size="small" class="text-center mt-2"
             v-if="isSuccess">Upload successfully! Time cost: {{ timeCost.toFixed(2) }}s
    </Message>
    <Message severity="error" size="small" class="mt-2 break-all"
             v-if="isFailed">{{ errorMsg }}
    </Message>
    <Button icon="pi pi-times" label="Cancel" @click="isOpen = false" class="mt-8 w-full "
            severity="secondary"></Button>
  </Dialog>
</template>