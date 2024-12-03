import { createApp } from "vue";
import App from "./App.vue";
import Antd from "ant-design-vue";
import 'ant-design-vue/dist/reset.css';
import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("./views/Home.vue"),
  }
  
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

//createApp(App).mount("#app");
createApp(App).use(router).use(Antd).mount("#app");
