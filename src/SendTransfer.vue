<script setup lang="ts">
import {listen} from "@tauri-apps/api/event";
import {computed, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";

const isOpen = defineModel()
const props = defineProps<{ files: FileDetail[] }>()

const isStart = ref(false);
const isSuccess = ref(false);
const isFailed = ref(false);

const isWaiting = computed(() => isStart.value && !isSuccess.value && !isFailed.value)

const timeCost = ref(0.0);
const errorMsg = ref('');

const currFile = ref({} as FileDetail);

watch(isOpen, async (newVal, _) => {
  if (newVal) {  // only when the dialog is opened
    if (props.files.length > 1) {
      currFile.value = await invoke("zip_files", {files: props.files})
    } else {
      currFile.value = props.files[0]
    }
    await invoke("upload_file", {file: currFile.value})
  }
})

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

listen<string>('zip_failed', (event) => {
  isFailed.value = true;
  errorMsg.value = event.payload;
})

</script>

<template>
  <Dialog v-model:visible="isOpen" modal :style="{width: '75%'}" :closable="false">
    <div class="text-xl mb-8">Uploading...</div>
    <ProgressBar mode="indeterminate" v-show="isWaiting"></ProgressBar>
    <div class="text-xs text-white/50 mt-2" v-show="isWaiting">Please wait for a while...</div>
    <Message severity="info" size="small" class="text-center mt-2" v-if="!isStart">
      Waiting the file to be zipped...
    </Message>
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