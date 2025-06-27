<script setup>
import LeftMenu from '../components/leftMenu.vue'
import BottomBar from '../components/BottomBar.vue'
import MainList from "../components/TypeList.vue"

import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { register } from '@tauri-apps/plugin-global-shortcut';

const registerKey = async () => {
}



onMounted(async () => {

})

</script>

<template>
    <div class="main-container">
        <!-- 左侧菜单 -->
        <LeftMenu class="left-menu" />

        <!-- 内容区域 -->
        <div class="main-content">
            <!-- 顶部内容区域（比如公告、按钮等）-->
            <div class="header-section">
                <slot name="header" />
            </div>

            <!-- 主内容区域（阵容列表） -->
            <div class="body-section">
                <slot name="body" />
               <router-view />
            </div>

            <!-- 底部栏 -->
            <BottomBar />
        </div>
    </div>
</template>

<style scoped>
.main-container {
    display: flex;
    height: 100vh;
    background-color: #0e1220;
    overflow: hidden;
}

.left-menu {
    width: 220px;
    background-color: rgb(16, 21, 32);
    flex-shrink: 0;
    margin-top: 2vh;
    margin-left: 4vh;
    margin-bottom: 2vh;
    border-radius: 5vh;
}

.main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 16px;
    overflow: hidden;
}

.header-section {
    flex-shrink: 0;
}

.body-section {
    flex: 1;
    overflow-y: auto;
    margin: 16px 0;
}
</style>
