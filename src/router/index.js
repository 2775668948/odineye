// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router';
import Home from "../views/Home.vue"

import Record from "../views/Record.vue"

import GameStart from "../views/GameStart.vue"
import { path } from '@tauri-apps/api';

import Gameing from '../views/Gameing.vue';

import MainList from "../components/TypeList.vue"

// lol的主要界面
import LolMain from "../views/lolMain.vue"

// 电子竞技界面
import esports from "../views/esports.vue"

// 英雄排行界面
import rank from "../views/rank.vue"

// 战绩看板界面
import board from "../views/board.vue"


// 战绩查询界面
import search from "../views/search.vue"

// 更多界面
import more from "../views/more.vue"

const routes = [
    {
        path: '/',
        component: Home,
        children: [
            {
                path: '',
                // 不要给 LolMain 本身起名字，避免和默认子路由冲突
                name: 'Home',
                component: LolMain,
                children: [
                    {
                        path: '',
                        name: 'esports',        // 默认子路由命名
                        component: esports
                    },
                    {
                        path: 'rank',
                        name: 'rank',
                        component: rank
                    },
                    {
                        path: 'board',
                        name: 'board',
                        component: board
                    },
                    {
                        path: 'search',
                        name: 'search',
                        component: search
                    },
                    {
                        path: 'more',
                        name: 'more',
                        component: more
                    }
                ]
            },
            {
                path: 'tftMain',       // 去掉前面 `/`，因为这是子路由
                name: 'tftMain',
                component: MainList
            }
        ]
    },
    {
        path: '/record',
        name: 'Record',
        component: Record
    },
    {
        path: '/gamestart',
        name: 'GameStart',
        component: GameStart
    },
    {
        path: '/gameing',
        name: 'Gameing',
        component: Gameing
    }
];


const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
