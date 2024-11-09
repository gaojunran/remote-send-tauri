<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import SendTransfer from "./SendTransfer.vue";
import {load, Store} from "@tauri-apps/plugin-store";
import FileDisplay from "./FileDisplay.vue";
import RecvTransfer from "./RecvTransfer.vue";
import {listen} from "@tauri-apps/api/event";
import {open} from "@tauri-apps/plugin-dialog";
import {store} from "./main.ts";


// states
const bucketName = ref("");
const region = ref("");
const accessKey = ref("");
const secretKey = ref("");
const endpoint = ref("");
const isAutoLaunch = ref(false);
const downloadTarget = ref("");

const isStorageConfigured = computed(() => {
  return bucketName.value && region.value && endpoint.value && accessKey.value && secretKey.value
});

const isSendPanelOpen = ref(false);
const isRecvPanelOpen = ref(false);
const isSettingsPanelOpen = ref(false);

const isSaveSettingsMsgVisible = ref(false);
const isPeekLatestLoading = ref(false);

const isSendDialogOpen = ref(false);
const isRecvDialogOpen = ref(false);

const hasSendFile = computed(() => !!sendFile.value);
const hasRecvFile = computed(() => !!recvFile.value);

const sendFile = ref(null as FileDetail | null);
const recvFile = ref(null as ObjectDetail | null);
const globalError = ref("");

const pickFile = async () => {
  sendFile.value = await invoke("pick_file", {})
}

const saveSettings = async () => {
  await store.set("region", region.value)
  await store.set("endpoint", endpoint.value)
  await store.set("bucket_name", bucketName.value)
  await store.set("access_key", accessKey.value)
  await store.set("secret_key", secretKey.value)
  await store.set("auto_launch", isAutoLaunch.value)
  await store.set("download_target", downloadTarget.value)
  await store.save();
  isSaveSettingsMsgVisible.value = true;
  setTimeout(() => {
    isSaveSettingsMsgVisible.value = false;
  }, 3500);
}

const getSettings = async () => {
  bucketName.value = await store.get<string>("bucket_name") || "";
  region.value = await store.get<string>("region") || "";
  endpoint.value = await store.get<string>("endpoint") || "";
  accessKey.value = await store.get<string>("access_key") || "";
  secretKey.value = await store.get<string>("secret_key") || "";
  isAutoLaunch.value = await store.get<boolean>("auto_launch") || false;
  downloadTarget.value = await store.get<string>("download_target") || "";
}

const beginSend = async () => {
  isSendDialogOpen.value = true;
  await invoke("upload_file", {file: sendFile.value})
}


const beginRecv = async () => {
  isRecvDialogOpen.value = true;
}

const expandSendPanel = () => {
  isSendPanelOpen.value = true;
  isRecvPanelOpen.value = false;
  isSettingsPanelOpen.value = false;
}

const expandRecvPanel = () => {
  isRecvPanelOpen.value = true;
  isSendPanelOpen.value = false;
  isSettingsPanelOpen.value = false;
}

const expandSettingsPanel = () => {
  isSettingsPanelOpen.value = true;
  isSendPanelOpen.value = false;
  isRecvPanelOpen.value = false;
}

const pickDownloadTarget = async () => {
  const result = await open({directory: true})
  if (result) downloadTarget.value = result
}

const peekLatestFile = async () => {
  isPeekLatestLoading.value = true;
  recvFile.value = await invoke("peek_latest_file", {})
  isPeekLatestLoading.value = false;
}

listen<string>('glob_error', (event) => {
  globalError.value = event.payload
})

onMounted(async () => {
  await getSettings()
  if (isStorageConfigured) {
    expandSendPanel()
  } else {
    expandSettingsPanel()
  }
})
</script>


<template>
  <SendTransfer v-model="isSendDialogOpen"></SendTransfer>
  <RecvTransfer :object="recvFile || undefined" v-model="isRecvDialogOpen"></RecvTransfer>
  <div class="p-4">
    <Message severity="error" closable @close="globalError = ''" v-if="globalError" variant="outlined" class="mb-4">
      {{ globalError }}
    </Message>
    <Panel toggleable header="Send" :collapsed="!isSendPanelOpen"
           @toggle="expandSendPanel"
    >
      <div class="">
        <Button icon="pi pi-file-plus" label="Pick a File" severity="secondary"
                class="w-full flex justify-center" @click="pickFile()"
        ></Button>
        <Button icon="pi pi-clipboard" label="From Clipboard" severity="secondary"
                class="mt-2 w-full flex justify-center"
        ></Button>
        <FileDisplay :name="sendFile?.name" :size="sendFile?.size" v-if="hasSendFile" class="mt-2"/>

        <Button icon="pi pi-check" label="Send" severity="success"
                class="mt-2 w-full flex justify-center" v-if="hasSendFile" @click="beginSend()"
        ></Button>

      </div>
    </Panel>

    <div class="py-2"></div>

    <Panel toggleable header="Receive" :collapsed="!isRecvPanelOpen"
           @toggle="expandRecvPanel"
    >
      <div class="">
        <Button icon="pi pi-download"
                label="Peek Latest File"
                :loading="isPeekLatestLoading"
                severity="secondary"
                class="w-full flex justify-center" @click="peekLatestFile()"
        ></Button>
        <FileDisplay :name="recvFile?.key" :size="recvFile?.size" :last-time="recvFile?.last_modified"
          v-if="hasRecvFile" class="mt-2"
        ></FileDisplay>
        <Button icon="pi pi-check" label="Receive" severity="success"
                class="mt-2 w-full flex justify-center" v-if="hasRecvFile" @click="beginRecv()"
        ></Button>
      </div>
    </Panel>

    <div class="py-2"></div>


    <Panel toggleable header="Settings" :collapsed="!isSettingsPanelOpen"
           @toggle="expandSettingsPanel()"
    >
      <Message severity="warn" size="small" class="mb-2"
               v-if="!isStorageConfigured">Configure S3 Storage before you start!
      </Message>
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

      <div class="flex justify-between items-center mt-4 text-sm">
        <div>
          <div>Download Target</div>
          <div class="text-xs text-gray-500 break-all mr-4">{{ downloadTarget }}</div>
        </div>
        <Button severity="secondary" @click="pickDownloadTarget()" class="text-xs px-3">Select</Button>
      </div>

      <Button icon="pi pi-save" label="Save" severity="success"
              class="mt-4 w-full" @click="saveSettings()" :disabled="isSaveSettingsMsgVisible"></Button>
      <Message severity="success" size="small" class="text-center mt-2"
               :life="3000" v-if="isSaveSettingsMsgVisible">Save settings successfully!
      </Message>
    </Panel>
  </div>
</template>