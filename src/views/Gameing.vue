<template>
    <canvas ref="canvas" class="fixed top-0 left-0 w-screen h-screen pointer-events-none z-50" />
</template>


<script setup>


import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const canvas = ref(null)


onMounted(async () => {
    // 调用 Rust 命令，设置点击穿透
    console.log("组件开始加载")
    await invoke("closegmwindow2")
    const el = canvas.value
    if (!el) return

    // 设置 canvas 尺寸
    el.width = window.innerWidth
    el.height = window.innerHeight
    const ctx = el.getContext('2d')

    // 示例：绘制一个高亮框（你可以动态修改位置/尺寸）
    ctx.strokeStyle = 'rgba(255, 255, 0, 0.8)'
    ctx.lineWidth = 1
    ctx.strokeRect(window.innerWidth / 2 - 50, window.innerHeight / 2 - 75, 100, 150)
})


</script>

<style scoped>
canvas {
    pointer-events: none;
    /* 允许点击穿透 */
}

.test {
    background-color: transparent;
    margin-top: 20px;
    height: 100vh;
    width: 100vw;
}
</style>