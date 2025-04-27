<template>
    <div class="bottom-bar">
        <!-- 左边：辅助设置 -->
        <button class="btn setting-btn">
            <img src="../assets/setting.png" alt="设置" />
            辅助设置
        </button>

        <n-button @click="handleClick">
            点它
        </n-button>



        <!-- 右边：开始游戏 + 客服 -->
        <div class="right-section">
            <n-tooltip trigger="hover" placement="top">
                <template #trigger>
                    <button class="start-btn" @click="startLol">开始游戏</button>
                </template>
                跳过WeGame直接启动LOL
            </n-tooltip>
        </div>
    </div>
</template>

<script setup>
// 暂无特殊逻辑
import { NTooltip } from 'naive-ui'

import { WebviewWindow } from "@tauri-apps/api/webviewWindow"


const startLol = () => {
    console.log("启动lol")
    // invoke('start_lol').then((message) => console.log(message)).catch((error) => console.error(error));
}

const handleClick = () => {
    console.log("打开记牌器...")

    const webview = new WebviewWindow('record', {
        label: "record",
        width: 400,
        height: 200,
        url: "#/record",
        decorations: true
    })
    webview.once('tauri://created', function () {
        // webview successfully created
    });
    webview.once('tauri://error', function (e) {
        // an error happened creating the webview
        console.log(e)
    });
}

</script>

<style scoped>
.bottom-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: #0e1220;
    padding: 12px 20px;
    border-radius: 20px;
    height: 60px;
    width: 100%;
    box-sizing: border-box;
}

/* 左侧设置按钮 */
.setting-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #20273a;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 20px;
    font-weight: bold;
    cursor: pointer;
}

.setting-btn img {
    width: 20px;
    height: 20px;
}

/* 中间下拉 */
.selectors {
    display: flex;
    gap: 12px;
}

.select {
    background: #0e1220;
    color: #ccc;
    border: none;
    padding: 6px 14px;
    border-radius: 20px;
    font-size: 14px;
    cursor: pointer;
    appearance: none;
    position: relative;
    font-weight: bold;
}

/* 右侧操作按钮 */
.right-section {
    display: flex;
    align-items: center;
    gap: 12px;
}

.start-btn {
    background-color: rgb(93, 246, 255);
    color: black;
    font-weight: bold;
    padding: 12px 24px;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    font-size: 16px;
}

.start-btn:hover {
    background-color: #0ff;
}

.support-btn {
    background: rgb(93, 246, 255);
    width: 40px;
    height: 40px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
}

.support-btn img {
    width: 20px;
    height: 20px;
}
</style>