import './style.css';
import "primeicons/primeicons.css"
import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import {Button, Dialog, Fieldset, Panel, ProgressBar,
    FloatLabel, InputText, Divider, ToggleSwitch, Password, Message} from "primevue";
import {load} from "@tauri-apps/plugin-store";


const app = createApp(App);
app.use(PrimeVue, {
    theme: {
        preset: Aura,
    }
});
app.component("Button", Button);
app.component("Fieldset", Fieldset);
app.component("Panel", Panel)
app.component("Dialog", Dialog)
app.component("ProgressBar", ProgressBar)
app.component("FloatLabel", FloatLabel)
app.component("InputText", InputText)
app.component("Divider", Divider)
app.component("ToggleSwitch", ToggleSwitch)
app.component("Password", Password)
app.component("Message", Message)

app.mount("#app");

// export const store = await load("store.json", {autoSave: true});
//
//
// if (await store.get<boolean>("auto_launch")) {
//     await enable();
// } else {
//     await disable();
// }

