<script setup lang="ts">
import {computed, onMounted, provide, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import SendTransfer from "./SendTransfer.vue";
import {load, Store} from "@tauri-apps/plugin-store";
import FileDisplay from "./FileDisplay.vue";
import RecvTransfer from "./RecvTransfer.vue";
import {listen} from "@tauri-apps/api/event";
import {open} from "@tauri-apps/plugin-dialog";
import {isText} from "./utils.ts";

let store: {
  content: Store | null;
} = {
  content: null
};  // object using `new` is not well compatible with Vue `ref`, so using an object to wrap it.

provide("store", store);  // provide store to all children components

// states
const bucketName = ref("");
const region = ref("");
const accessKey = ref("");
const secretKey = ref("");
const endpoint = ref("");
const isAutoLaunch = ref(false);
const downloadTarget = ref("");
const isDark = ref(false);

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
const isTextDialogOpen = ref(false);
const isAboutUsDialogOpen = ref(false);

const sendFiles = ref([] as FileDetail[]);  // allow multiple files
const recvFile = ref(null as ObjectDetail | null);

const hasSendFile = computed(() => !!sendFiles.value.length);
const hasRecvFile = computed(() => !!recvFile.value);
const textContent = ref("");
const globalError = ref("");

const pickFiles = async () => {
  const pickedFiles: FileDetail[] = await invoke("pick_files", {});
  pickedFiles.filter(f => !sendFiles.value.includes(f)).forEach(f =>
      sendFiles.value.push(f)  // action: duplicate files are ignored
  )
}

const saveSettings = async () => {
  // apply dark mode
  isDark.value ? document.documentElement.classList.add('my-app-dark')
      : document.documentElement.classList.remove('my-app-dark');

  await store.content?.set("region", region.value)
  await store.content?.set("endpoint", endpoint.value)
  await store.content?.set("bucket_name", bucketName.value)
  await store.content?.set("access_key", accessKey.value)
  await store.content?.set("secret_key", secretKey.value)
  await store.content?.set("auto_launch", isAutoLaunch.value)
  await store.content?.set("download_target", downloadTarget.value)
  await store.content?.set("is_dark", isDark.value)
  await store.content?.save();
  // await getSettings();
  isSaveSettingsMsgVisible.value = true;
  setTimeout(() => {
    isSaveSettingsMsgVisible.value = false;
  }, 2000);
}

const getSettings = async () => {
  bucketName.value = await store.content?.get<string>("bucket_name") || "";
  region.value = await store.content?.get<string>("region") || "";
  endpoint.value = await store.content?.get<string>("endpoint") || "";
  accessKey.value = await store.content?.get<string>("access_key") || "";
  secretKey.value = await store.content?.get<string>("secret_key") || "";
  isAutoLaunch.value = await store.content?.get<boolean>("auto_launch") || false;
  downloadTarget.value = await store.content?.get<string>("download_target") || "";
  isDark.value = await store.content?.get<boolean>("is_dark") || false;
}

const beginSend = async () => {
  isSendDialogOpen.value = true;
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

const textToFile = async () => {
  const file: FileDetail = await invoke("text_to_file", {text: textContent.value});
  // remove the last text file if exists
  sendFiles.value = sendFiles.value.filter(f => !isText(f.name));
  sendFiles.value.push(file);
  isTextDialogOpen.value = false;
}

const popFile = (file: FileDetail) => {
  const index = sendFiles.value.indexOf(file);
  if (index !== -1) {
    sendFiles.value.splice(index, 1);
  }
}

watch(isDark, (newV, _) => {  // for dynamically previewing dark/light mode
  newV ? document.documentElement.classList.add('my-app-dark')
      : document.documentElement.classList.remove('my-app-dark');
})

listen<string>('glob_error', (event) => {
  globalError.value = event.payload
})

onMounted(async () => {
  store.content = await load("store.json", {autoSave: true})
  await getSettings()

  // // apply auto launch
  // if (isAutoLaunch.value) {
  //   await enable();
  // } else {
  //   await disable();
  // }

  // expand panel based on whether the storage is configured or not
  if (isStorageConfigured) {
    expandSendPanel()
  } else {
    expandSettingsPanel()
  }

  // apply dark mode on start up
  isDark.value ? document.documentElement.classList.add('my-app-dark')
      : document.documentElement.classList.remove('my-app-dark');
})
</script>


<template>
  <SendTransfer :files="sendFiles" v-model="isSendDialogOpen"></SendTransfer>
  <RecvTransfer :object="recvFile || undefined" v-model="isRecvDialogOpen"></RecvTransfer>
  <!-- Add Text Dialog -->
  <Dialog v-model:visible="isTextDialogOpen" header="Text" modal :style="{width: '75%'}">
    <Textarea v-model="textContent" class="w-full" rows="6"></Textarea>
    <div class="text-xs text-gray-500">You can only attach texts as one file at a time.</div>
    <Button label="Save" severity="primary" icon="pi pi-check"
            class="mt-2 w-full" @click="textToFile()"></Button>
  </Dialog>
  <!-- About Us Dialog -->
  <Dialog v-model:visible="isAboutUsDialogOpen" header="About Us" modal :style="{width: '75%'}">
    <div class="">
      <p class="text-sm">
        Note: RemoteSend does not retain any data on its server. All client requests are managed exclusively through your designated S3 service provider.
      </p>
      <Divider class="py-2"/>
      <p>
        See more on <a href="https://github.com/gaojunran/remote-send-tauri" target="_blank" class="text-blue-500">GitHub</a>
      </p>

      <p class="mt-2">
        Built with <a href="https://tauri.studio/" target="_blank" class="text-blue-500">Tauri</a>,
        <a href="https://vuejs.org/" target="_blank" class="text-blue-500">Vue3</a>,
        <a href="https://tailwindcss.com/" target="_blank" class="text-blue-500">Tailwind CSS</a>,
        <a href="https://primevue.org" target="_blank" class="text-blue-500">PrimeVue</a>.
      </p>
    </div>
  </Dialog>

  <!-- Main Panel -->
  <div class="p-4">
    <Message severity="error" closable @close="globalError = ''" v-if="globalError" variant="outlined"
             class="mb-4 break-all">
      {{ globalError }}
    </Message>
    <Panel toggleable header="Send" :collapsed="!isSendPanelOpen"
           @toggle="expandSendPanel"
    >
      <div class="">
        <div class="flex justify-between items-center mb-2">
          <Button icon="pi pi-pen-to-square" label="Text" severity="secondary"
                  class="w-1/2 flex justify-center mr-2" @click="isTextDialogOpen = true"
          ></Button>
          <Button icon="pi pi-image" label="Image" severity="secondary"
                  class="w-1/2 flex justify-center" @click="pickFiles()"
          ></Button>
        </div>
        <Button icon="pi pi-file-plus" label="Add Files..." severity="secondary"
                class="mt-2 w-full flex justify-center" @click="pickFiles()"
        ></Button>

        <FileDisplay
            v-for="item in sendFiles" :key="item.name"
            :name="item?.name" :size="item?.size"  :text="textContent" v-if="hasSendFile" class="mt-2"
            :allow-delete="true" @delete="popFile(item)"
        />

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
        <Button icon="pi pi-search"
                label="Peek..."
                :loading="isPeekLatestLoading"
                severity="secondary"
                class="w-full flex justify-center" @click="peekLatestFile()"
        ></Button>
        <FileDisplay :name="recvFile?.key ?? ''" :size="recvFile?.size" :last-time="recvFile?.last_modified"
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

      <div class="flex justify-between items-center mt-4">
        <div>Dark Mode</div>
        <ToggleSwitch checked v-model="isDark"></ToggleSwitch>
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
               :life="1500" v-if="isSaveSettingsMsgVisible">Save settings successfully!
      </Message>

      <Button icon="pi pi-info-circle" label="About Us" @click="isAboutUsDialogOpen = true" severity="secondary" class="w-full mt-4" />

    </Panel>
  </div>
</template>