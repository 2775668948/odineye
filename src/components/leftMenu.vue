<template>
    <aside class="left-menu">
        <!-- Logo 和名称 -->
        <div class="logo-section">
            <img src="../assets/logo.png" alt="Logo" class="logo-img" />
            <div class="logo-title">
                Violet
                <span class="crown">👑</span>
            </div>
        </div>

        <!-- 蓝色分隔条 -->
        <div class="divider"></div>

        <!-- 菜单项 -->
        <nav class="menu">
            <div class="menu-item" :class="{ active: activeMenu === 'lol' }" @click="activeMenu = 'lol'">
                <img src="../assets/lol.png" class="icon" alt="英雄联盟" />
                <span>英雄联盟</span>
            </div>
            <div class="menu-item" :class="{ active: activeMenu === 'tft' }" @click="activeMenu = 'tft'">
                <img src="../assets/lol.png" class="icon" alt="云顶之弈" />
                <span>云顶之弈</span>
            </div>
        </nav>



        <!-- 底部链接 -->
        <div class="footer">联系作者</div>
    </aside>
</template>

<script setup>
import { ref } from 'vue'

const activeMenu = ref('lol') // 默认选中英雄联盟
</script>

<style scoped>
.left-menu {
    width: 260px;
    min-height: 100vh;
    background-color: red;
    color: white;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 20px;
    box-sizing: border-box;
}

.logo-section {
    text-align: center;
    margin-bottom: 30px;
}

.logo-img {
    width: 80px;
    margin-bottom: 10px;
}

.logo-title {
    font-size: 18px;
    font-weight: bold;
}

.crown {
    font-size: 14px;
    background-color: #5252b8;
    padding: 2px 6px;
    border-radius: 5px;
    margin-left: 6px;
}

.divider {

    margin-bottom: 20px;
}

.menu {
    width: 100%;
    padding: 0 20px;
    box-sizing: border-box;
}

.menu-item {
    display: flex;
    align-items: center;
    padding: 12px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: bold;
    color: white;
    transition: background 0.2s;
}

.menu-item:hover {
    background-color: #1B1D2A;
}

.menu-item.active span {
    color: #00f0ff;
}

.icon {
    width: 24px;
    height: 24px;
    margin-right: 10px;
}

.card {
    background-color: #00c2d4;
    border-radius: 12px;
    padding: 20px 10px;
    width: 200px;
    text-align: center;
    position: relative;
    margin-top: 40px;
}

.invite-tag {
    position: absolute;
    top: -12px;
    left: 10px;
    background-color: white;
    color: #00c2d4;
    font-size: 12px;
    padding: 2px 8px;
    border-radius: 8px;
    font-weight: bold;
}

.poro-img {
    width: 80px;
    margin-top: 10px;
}

.card-title {
    margin-top: 10px;
    font-weight: bold;
    color: white;
}

.card-progress {
    font-size: 14px;
    margin-top: 6px;
    color: white;
}

.card-count {
    font-size: 18px;
    font-weight: bold;
    margin-top: 2px;
    color: white;
}

.footer {
    margin-top: auto;
    margin-bottom: 30px;
    font-weight: bold;
    font-size: 16px;
}
</style>