import { createApp } from "vue";
import App from "./App.vue";
import naive from 'naive-ui'

import router from "./router/index.js"
import store from './store/index.js';

const app = createApp(App)
app.use(router).use(naive).use(store).mount("#app")

