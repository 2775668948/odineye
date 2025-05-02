// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router';
import Home from "../views/Home.vue"

import Record from "../views/Record.vue"

import GameStart from "../views/GameStart.vue"
import { path } from '@tauri-apps/api';

import Gameing from '../views/Gameing.vue';

const routes = [
    {
        path: '/',       // 访问 localhost:端口/ 时显示
        name: 'Home',
        component: Home
    },
    {
        path: '/record',       // 访问 localhost:端口/ 时显示
        name: 'Record',
        component: Record
    },
    {
        path: "/gamestart",
        name: "GameStart",
        component: GameStart
    },
    {
        path: "/gameing",
        name: "Gameing",
        component: Gameing
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
