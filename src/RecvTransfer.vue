<script setup lang="ts">

import {formatBytes, isText, isZipped, pathConcat} from "./utils.ts";

const props = defineProps<{
  object?: ObjectDetail;
}>()

import {listen} from "@tauri-apps/api/event";
import {computed, inject, ref, watch} from "vue";
import {Channel, invoke} from "@tauri-apps/api/core";
import {Store} from "@tauri-apps/plugin-store";
import FileDisplay from "./FileDisplay.vue";

const store: {
  content: Store | null;
} = inject("store") || { content: null };

const isOpen = defineModel<boolean>()

const channel = new Channel<ChannelBytes>();
channel.onmessage = (msg) => {
  bytesNow.value = msg.value;
}

watch(isOpen, async (newVal, _) => {
  if (newVal) {  // only when the dialog is opened, start downloading
    await invoke("download_file", {
      object: props.object,
      event: channel
    })
  }
})

const isStart = ref(false);
const isSuccess = ref(false);
const isFailed = ref(false);

const isWaiting = computed(() => isStart.value && !isSuccess.value && !isFailed.value)

const timeCost = ref(0.0);
const errorMsg = ref('');

const bytesNow = ref(0);
const percentNow = computed(() =>
    Math.ceil((bytesNow.value / (props.object?.size ?? 0)) * 100)
)

const currFiles = ref([] as FileDetail[])
const textContent = ref(null as string | null);

const fileToText = async (file: FileDetail | undefined) => {
  if (file) textContent.value = await invoke("file_to_text", {file: file});
}

listen<number>('download_started', (_) => {
  isStart.value = true;
  isSuccess.value = false;
  isFailed.value = false;
})

listen<number>('download_success', async (event) => {
  if (isZipped(props.object?.key!!)) {
    currFiles.value = await invoke("unzip_files", {object: props.object});
  } else {
    currFiles.value = [{
      path: pathConcat(await store.content?.get<string>("download_target") || "", props.object!!.key),
      name: props.object!!.key,
      size: props.object!!.size,
    }]
  }
  await fileToText(currFiles.value.find(f => isText(f.name))); // get the text content to display
  console.log(currFiles.value)
  timeCost.value = event.payload;
  bytesNow.value = props.object?.size ?? 0;
  isSuccess.value = true;
})

listen<string>('download_failed', (event) => {
  isFailed.value = true;
  errorMsg.value = event.payload;
})

listen<string>('unzip_failed', (event) => {
  isFailed.value = true;
  errorMsg.value = event.payload;
})

const reset = () => {
  isStart.value = false;
  isSuccess.value = false;
  isFailed.value = false;
  bytesNow.value = 0;
  currFiles.value = [];
  textContent.value = null;
  timeCost.value = 0.0;
  isOpen.value = false;
}

// only available when there is ONE file sent
// const openFile = async () => {
//   let parent = await store.content?.get<string>("download_target");
//   let file = props.object?.key;
//   await invoke("open", {
//     path: pathConcat(parent!!, file!!),
//   })
// }

const openFolder = async () => {
  let path = await store.content?.get<string>("download_target");
  await invoke("open", {
    path: path!!
  });
}

</script>

<template>
  <Dialog v-model:visible="isOpen" modal :style="{width: '75%'}" :closable="false">
    <div class="text-xl mb-8" v-if="isWaiting">Downloading...</div>
    <ProgressBar mode="determinate" :value="percentNow" v-show="isWaiting"></ProgressBar>
    <div class="text-xs text-white/50 mt-2" v-show="isWaiting">
      {{ formatBytes(bytesNow) }} / {{ formatBytes(props.object?.size) }}
    </div>
    <Message severity="success" size="small" class="text-center mt-2 mb-4"
             v-if="isSuccess">Download successfully! Time cost: {{ timeCost.toFixed(2) }}s
    </Message>
    <Message severity="error" size="small" class="mt-2 break-all mb-2"
             v-if="isFailed">{{ errorMsg }}
    </Message>
    <FileDisplay v-for="file in currFiles" :key="file.path" v-if="isSuccess"
                 :name="file.name" :size="file.size" :text="textContent || undefined" class="mt-2"
                 :allow-copy-text="true"
    ></FileDisplay>
    <div class="mt-4 flex" v-if="isSuccess">
<!--      <Button severity="info" icon="pi pi-file-check" label="Open" class="mr-1 text-xs" @click="openFile()"></Button>-->
      <Button severity="secondary" icon="pi pi-folder" label="Open Folder" @click="openFolder()" class="w-full"></Button>
    </div>
    <Button icon="pi pi-times" label="Cancel" @click="reset()" class="mt-2 w-full"
            severity="secondary"></Button>
  </Dialog>
</template>