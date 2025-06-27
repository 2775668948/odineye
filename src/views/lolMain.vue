<template>
    <div>
        <div class="top">
            <div class="topMenu">
                <img src="../assets/home.png" alt="" class="topmenuimg">
                <div class="topmenuText">
                    <span>电子竞技</span>
                </div>
            </div>
            <div class="topMenu">
                <img src="../assets/home.png" alt="" class="topmenuimg">
                <div class="topmenuText">
                    <span>英雄排行</span>
                </div>
            </div>
            <div class="topMenu">
                <img src="../assets/home.png" alt="" class="topmenuimg">
                <div class="topmenuText">
                    <span>战绩看板</span>
                </div>
            </div>
            <div class="topMenu">
                <img src="../assets/home.png" alt="" class="topmenuimg">
                <div class="topmenuText">
                    <span>战绩查询</span>
                </div>
            </div>
            <div class="topMenu">
                <img src="../assets/home.png" alt="" class="topmenuimg">
                <div class="topmenuText">
                    <span>设置</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>

// 自定义高亮色、背景等
const menuTheme = {
    itemTextColorActiveHorizontal: '#0ff', // 高亮字体颜色（青蓝色）
    itemTextColorHoverHorizontal: '#0ff',
    itemColorHoverHorizontal: 'transparent',
    itemColorActiveHorizontal: 'transparent'
}
</script>


<style scoped>
.top {
    height: 30px;
    /* background-color: red; */
    display: flex;
    justify-content: flex-start;

}

.topMenu {
    display: flex;
    width: 130px;
    margin-right: 20px;
}

.topmenuimg {
    height: 24px;
    width: 24px;
    margin-top: 1px;
}

/* #50546F */

.topmenuText {
    margin-left: 30px;
    color: #50546F;
    font-size: 16px;

}
</style>
