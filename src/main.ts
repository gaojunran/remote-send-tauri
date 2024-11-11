import './style.css';
import "primeicons/primeicons.css"
import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import {Button, Dialog, Fieldset, Panel, ProgressBar,
    FloatLabel, InputText, Divider, ToggleSwitch, Password, Message, Textarea} from "primevue";

const app = createApp(App);
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            darkModeSelector: ".my-app-dark"
        }
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
app.component("Textarea", Textarea)

app.mount("#app");



