<template>
    <div class="top">
        <div v-for="item in menuItems" :key="item.name" class="topMenu" :class="{ active: activeMenu === item.name }"
            @click="setActiveMenu(item.name, item.routeName)">
            <img :src="item.icon" alt="" class="topmenuimg" />
            <div class="topmenuText">
                <span>{{ item.label }}</span>
            </div>
        </div>
    </div>
    <div>
        <router-view />
    </div>
</template>


<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import iconEsports from '../assets/qizi.png'
import iconRank from '../assets/xinpaihang.png'
import iconBoard from '../assets/xunzhang.png'
import iconSearch from '../assets/search.png'
import iconMore from '../assets/more.png'
const router = useRouter()

// 默认激活项
const activeMenu = ref('电竞')

// 菜单配置项
const menuItems = [
    { name: '电竞', label: '电子竞技', icon: iconEsports, routeName: 'esports' },
    { name: '排行', label: '英雄排行', icon: iconRank, routeName: 'rank' },
    { name: '看板', label: '战绩看板', icon: iconBoard, routeName: 'board' },
    { name: '查询', label: '战绩查询', icon: iconSearch, routeName: 'search' },
    { name: '更多', label: '更多', icon: iconMore, routeName: 'more' }
]


// 切换激活菜单并跳转
function setActiveMenu(name, route) {
    activeMenu.value = name
    console.log("跳转去-->" + name)
    console.log(route)
    router.push({ name: route }) // ✅ 用 name 跳转更稳
}
</script>


<style scoped>
.top {
    height: 30px;
    display: flex;
    justify-content: flex-start;
}

.topMenu {
    display: flex;
    width: 130px;
    margin-right: 20px;
    cursor: pointer;
    transition: transform 0.2s ease;
    user-select: none;
}

.topmenuimg {
    height: 24px;
    width: 24px;
    margin-top: 1px;
    transition: transform 0.2s ease, filter 0.2s ease;
}

.topmenuText {
    position: relative;
    margin-left: 30px;
    color: #50546F;
    font-size: 16px;
    transition: color 0.3s ease, transform 0.2s ease;
}

.topmenuText::after {
    content: '';
    position: absolute;
    left: 0;
    bottom: -2px;
    width: 100%;
    height: 2px;
    background-color: white;
    transform: scaleX(0);
    transform-origin: left;
    transition: transform 0.3s ease;
}

.topMenu:hover {
    transform: scale(1.05);
}

.topMenu:hover .topmenuText {
    color: white;
    transform: scale(1.05);
}

.topMenu:hover .topmenuimg {
    transform: scale(1.1);
}

.topMenu:hover .topmenuText::after {
    transform: scaleX(1);
}

.topMenu:active {
    transform: scale(0.95);
}

.topMenu:active .topmenuimg {
    filter: brightness(0.8);
}

.topMenu:active .topmenuText {
    transform: scale(0.95);
    color: #ccc;
}

/* ✅ 激活状态：高亮文字颜色 + 下划线显示 */
.topMenu.active .topmenuText {
    color: #0ff;
}

.topMenu.active .topmenuText::after {
    transform: scaleX(1);
    background-color: #0ff;
}
</style>
