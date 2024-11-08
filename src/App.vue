<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import SendTransfer from "./SendTransfer.vue";
import {load, Store} from "@tauri-apps/plugin-store";

let store: Store;

const bucketName = ref("");
const region = ref("");
const accessKey = ref("");
const secretKey = ref("");
const endpoint = ref("");
const isAutoLaunch = ref(false);

const isSendPanelOpen = ref(false);
const isRecvPanelOpen = ref(false);
const isSettingsPanelOpen = ref(false);

const isSendDialogOpen = ref(false);
const isRecvDialogOpen = ref(false);

const hasSendFile = ref(false);

interface FileDetail {
  path: string;
  name: string;
  size: number;
}

const sendFile = ref(null as FileDetail | null);
const recvFile = ref(null as FileDetail | null);

const formatBytes = (bytes: number|undefined) => {
  if (bytes === undefined) {
    return '0 B';
  }
  const units = ['B', 'KB', 'MB', 'GB'];
  let index = 0;

  while (bytes >= 1024 && index < units.length - 1) {
    bytes /= 1024;
    index++;
  }

  return `${bytes.toFixed(2)} ${units[index]}`;
}

const pickFile = async () => {
  sendFile.value = await invoke("pick_file", {})
  hasSendFile.value = !!sendFile.value;
}

const saveSettings = async () => {
  await store.set("region", region.value)
  await store.set("endpoint", endpoint.value)
  await store.set("bucket_name", bucketName.value)
  await store.set("access_key", accessKey.value)
  await store.set("secret_key", secretKey.value)
  await store.set("auto_launch", isAutoLaunch.value)
}

const getSettings = async () => {
  bucketName.value = await store.get<string>("bucket_name") || "";
  region.value = await store.get<string>("region") || "";
  endpoint.value = await store.get<string>("endpoint") || "";
  accessKey.value = await store.get<string>("access_key") || "";
  secretKey.value = await store.get<string>("secret_key") || "";
  isAutoLaunch.value = await store.get<boolean>("auto_launch") || false;
}
onMounted(async () => {
  store = await load("store.json", {autoSave: true});
  await getSettings()
})
</script>


<template>
  <SendTransfer v-model="isSendDialogOpen"></SendTransfer>
  <div class="p-4">
    <Panel toggleable header="Send" :collapsed="!isSendPanelOpen"
           @toggle="isSendPanelOpen = !isSendPanelOpen"
    >
      <div class="">
        <Button icon="pi pi-file-plus" label="Pick a File" severity="secondary"
                class="w-full flex justify-center" @click="pickFile()"
        ></Button>
        <Button icon="pi pi-clipboard" label="From Clipboard" severity="secondary"
                class="mt-2 w-full flex justify-center"
        ></Button>

        <Panel class="mt-2 border-white/50 hover:border-white transition"
               v-if="hasSendFile">
          <div class="flex items-center justify-start">
            <div class="pi pi-file mr-4"></div>
            <div class="text-sm">
              <div>{{ sendFile?.name }}</div>
              <div class="text-xs text-gray-500">{{ formatBytes(sendFile?.size) }}</div>
            </div>

          </div>

        </Panel>

        <Button icon="pi pi-check" label="Send" severity="success"
                class="mt-2 w-full flex justify-center" v-if="hasSendFile" @click="isSendDialogOpen = true"
        ></Button>

      </div>
    </Panel>

    <div class="py-2"></div>

    <Panel toggleable header="Receive" :collapsed="!isRecvPanelOpen"
           @toggle="isRecvPanelOpen =!isRecvPanelOpen"
    >
      <div class="">
        <Button icon="pi pi-download" label="Check a File" severity="secondary"
                class="w-full flex justify-center"
        ></Button>
        <Button icon="pi pi-check" label="Receive" severity="success"
                class="mt-2 w-full flex justify-center"
        ></Button>
      </div>
    </Panel>

    <div class="py-2"></div>


    <Panel toggleable header="Settings" :collapsed="!isSettingsPanelOpen"
           @toggle="isSettingsPanelOpen =!isSettingsPanelOpen; getSettings()"
    >
      <div class="mb-4">S3 Storage</div>
      <FloatLabel variant="in" class="mt-2">
        <InputText id="region" v-model="region" class="w-full"></InputText>
        <label for="region">Region</label>
      </FloatLabel>
      <FloatLabel variant="in" class="mt-2">
        <InputText id="endpoint" v-model="endpoint" class="w-full"></InputText>
        <label for="endpoint">Endpoint</label>
      </FloatLabel>
      <FloatLabel variant="in" class="mt-2">
        <InputText id="bucket_name" v-model="bucketName" class="w-full"></InputText>
        <label for="bucket_name">Bucket Name</label>
      </FloatLabel>
      <FloatLabel variant="in" class="mt-2">
        <InputText id="access_key" v-model="accessKey" class="w-full"></InputText>
        <label for="access_key">Access Key</label>
      </FloatLabel>
      <FloatLabel variant="in" class="mt-2">
        <Password id="secret_key" :feedback="false" v-model="secretKey" class="w-full"></Password>
        <label for="secret_key">Secret Key</label>
      </FloatLabel>

      <Divider/>

      <div class="flex justify-between items-center mt-8">
        <div>Auto Launch</div>
        <ToggleSwitch checked v-model="isAutoLaunch"></ToggleSwitch>
      </div>

      <Button icon="pi pi-save" label="Save" severity="success" class="mt-4 w-full" @click="saveSettings()"></Button>
    </Panel>
  </div>
</template>