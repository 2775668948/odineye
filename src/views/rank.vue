<template>
    <div class="header-container">
        <!-- 搜索框区域 -->
        <div class="header-left">
            <div>
                <input type="text" placeholder="搜索" v-model="selectedMode" class="search" />
            </div>
        </div>
        <!-- 游戏模式下拉菜单 -->
        <div class="header-mid">
            <div>
                <span>{{ options[activeIndex] }}</span>
            </div>
            <div v-if="!isOpen">
                <img src="../assets/xiala.png" alt="" class="dropdown-icon" @click="isOpen = !isOpen">
            </div>
            <div v-if="isOpen">
                <img src="../assets/shangla.png" alt="" class="dropdown-icon" @click="isOpen = !isOpen">
            </div>
            <div v-if="isOpen" class="dropdown-panel">
                <div class="header-mid-content">
                    <div v-for="(item, idx) in options" :key="item" :class="{ active: activeIndex === idx }"
                        @click="activeIndex = idx; isOpen = false;">
                        {{ item }}
                    </div>
                </div>
            </div>
        </div>
        <!-- 只有选中排位模式时才展示分路下拉菜单 -->
        <div class="header-mid" v-if="activeIndex === 0">
            <div>
                <span>{{ langeOptions[langActiveIndex] }}</span>
            </div>
            <div v-if="!lanesOpen">
                <img src="../assets/xiala.png" alt="" class="dropdown-icon" @click="lanesOpen = !lanesOpen">
            </div>
            <div v-if="lanesOpen">
                <img src="../assets/shangla.png" alt="" class="dropdown-icon" @click="lanesOpen = !lanesOpen">
            </div>
            <div v-if="lanesOpen" class="dropdown-panel2">
                <div class="header-mid-content-lang">
                    <div v-for="(item, idx) in langeOptions" :key="item" :class="{ active: langActiveIndex === idx }"
                        @click="handleLaneChange(item, idx)">{{ item }}</div>
                </div>
            </div>
        </div>
    </div>
    <div>
        123
    </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { NSelect } from 'naive-ui';
import { FlashOutline } from "@vicons/ionicons5";

const selectedMode = ref('');
const selectedLane = ref('上路');
const selectedGameMode = ref('');

// 模式切换
const isOpen = ref(false);
const activeIndex = ref(0);

const langActiveIndex = ref(0);
const lanesOpen = ref(false);

const options = ['排位模式', '极地大乱斗', '无限火力'];
const langeOptions = ["上路", "中路", "下路", "打野", "辅助"];

const handleLaneChange = (item, idx) => {
    selectedLane.value = item;
    langActiveIndex.value = idx;
    lanesOpen.value = false;
};

const gameModeOptions = [
    { label: '排位模式', value: '' },
    { label: '极地大乱斗', value: '1' },
    { label: '无限火力', value: '2' }
];

const handleSearch = () => {
    console.log('搜索 button clicked!');
    console.log('Selected Mode:', selectedMode.value);
    console.log('Selected Lane:', selectedLane.value);
};
</script>

<style scoped>
.header-container {
    margin-top: 20px;
    display: flex;
}

.header-left {
    width: 120px;
    background-color: #101323;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    height: 28px;
    color: white;
    border: 0.5px solid #23243a;
}


.header-mid {
    width: 120px;
    background-color: #101323;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    height: 28px;
    color: white;
    border: 0.5px solid #23243a;
    margin-left: 30px;
    position: relative;
}

.header-left:focus-within {
    border: 1.5px solid #01E1EE;
}

.search {
    width: 80px;
    background-color: #101323;
    color: white;
    border: none;
    height: 18px;
    margin-left: 5px;
}


.header-mid-content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    overflow: hidden;
}

.header-mid-content-lang {
    width: 100%;
    height: 150px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    overflow: hidden;
}

.header-mid-content-lang>div {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    cursor: pointer;
    user-select: none;
    width: 100%;
    text-align: left;
    padding-left: 16px;
    font-size: 14px;
    line-height: 28px;
    transition: background 0.2s;
}

header-mid-content-lang>div {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    cursor: pointer;
    user-select: none;
    width: 100%;
    text-align: left;
    padding-left: 16px;
    font-size: 14px;
    line-height: 28px;
    transition: background 0.2s;
}

.header-mid-content>div {
    flex: 1;
    display: flex;
    align-items: flex-start;
    justify-content: flex-start;
    cursor: pointer;
    user-select: none;
    width: 100%;
    text-align: left;
    padding-left: 16px;
    font-size: 14px;
    line-height: 40px;
    transition: background 0.2s;
}

.header-mid-content>div.active {
    background: #01E1EE;
    color: #fff;
    border-radius: 6px;
}

.header-mid-content>div:hover {
    background: #405073;
    color: #fff;
}

.header-mid-content-lang>div.active {
    background: #01E1EE;
    color: #fff;
    border-radius: 6px;
}

.header-mid-content-lang>div:hover {
    background: #405073;
    color: #fff;
}


.search:focus {
    outline: none;
    box-shadow: none;
}

.search::placeholder {
    color: #fff;
    opacity: 1;
}

.dropdown-icon {
    width: 16px;
    height: 16px;
    margin-left: 15px;
    cursor: pointer;
    margin-top: 8px;
}

.dropdown-panel {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    background: #23243a;
    color: #fff;
    border-radius: 8px;
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.18);
    z-index: 10;
    min-height: 100px;
    height: 120px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: stretch;
    padding: 0;
}

.dropdown-panel2 {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    background: #23243a;
    color: #fff;
    border-radius: 8px;
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.18);
    z-index: 10;
    min-height: 140px;
    height: 140px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: stretch;
    padding: 0;

}
</style>