<template>
    <div class="outer-container" data-tauri-drag-region :class="{ 'hovered': isHovered }" @mouseenter="isHovered = true"
        @mouseleave="isHovered = false">
        <div class="content" data-tauri-drag-region>
            <div>
                <div class="topCon">
                    <div>
                        按Table键可以快速呼出
                    </div>
                    <div class="closeGameStartwin">
                        <img src="../assets/close.png" @click="closeGmWindow" alt="">
                    </div>
                </div>
                <div class="tableHead">
                    <!-- 上面的表格头部 -->

                    <div class="tableHeaditem">
                        <div>
                            召唤师ID
                        </div>
                    </div>
                    <div class="tableHeaditem">
                        <div>
                            段位
                        </div>
                    </div>
                    <div class="tableHeaditem">
                        <div>
                            最近常玩
                        </div>
                    </div>
                    <div class="tableHeaditem">
                        <div>
                            近20场平均排名
                        </div>
                    </div>

                </div>
                <div v-for="n in 7" :key="n">
                    <!-- 真正的排名 -->
                    <div class="avgHead">
                        <!-- 头像 -->
                        <div class="avgHeaditem bodyname">
                            <div>
                                <n-avatar round size="small" style="width: 20px; height: 20px;"
                                    src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg" />
                            </div>
                            <div class="palyername">
                                <span>Violet</span>
                            </div>
                        </div>
                        <div class="avgHeaditem duanwei">
                            <div>
                                <img src="https://s-tft-api.op.gg/img/set/14/tft-regalia/TFT_Regalia_Diamond.png?image=q_auto:good,f_webp,w_144&b=YsZLcJi1FhZQSJir-fEkn"
                                    alt="">
                            </div>
                            <div>
                                <span>永恒钻石</span>
                            </div>
                        </div>
                        <div class="avgHeaditem changwan">
                            <div class="changwans">
                                <div class="changwanname">
                                    街头恶魔 布兰德
                                </div>
                                <div>
                                    |
                                </div>
                            </div>
                            <div>
                                <div class="wanjiatag">
                                    <n-tag type="success" size="small" round @close="handleClose">
                                        上等马
                                    </n-tag>
                                </div>
                            </div>
                        </div>
                        <div class="avgHeaditem avgranks">
                            <div class="avgbuju">
                                <div class="avgrank">
                                    <span>#1.3</span>
                                </div>
                            </div>

                            <div class="showrank">
                                <!-- <img src="../assets/left.png" alt=""> -->
                                <div>
                                    < </div>
                                </div>
                            </div>

                        </div>
                    </div>
                </div>


            </div>
        </div>
</template>


<script setup>
// import { appWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/core";


async function closeGmWindow() {
    try {
        await invoke('closegmwindow');
        console.log('窗口关闭命令已发送');
    } catch (error) {
        console.error('关闭窗口失败:', error);
    }
}



</script>

<style scoped>
.outer-container {
    width: 100vw;
    height: 100vh;
    /* border-radius: 20px; */
    /* background: linear-gradient(to bottom,
            rgba(30, 30, 30, 0.8),
            rgba(30, 30, 30, 0.3)); */
    overflow: hidden;
    -webkit-app-region: drag;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    transition: all 0.3s ease;
    background-color: rgb(15, 16, 26);
}

.outer-container.hovered {
    /* background: linear-gradient(to bottom,
            rgba(50, 50, 50, 0.85),
            rgba(50, 50, 50, 0.4)); */
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.4);
}

.content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    /* ⚡关键：这里不设置 no-drag，这样内容也能被拖动 */
    /* 如果你有按钮、输入框，才需要局部 no-drag */
    color: white;
    font-size: 16px;
}

.topCon {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
    height: 20px;
    margin-top: 10px;
    margin-left: 10px;
    margin-right: 10px;
}

.closeGameStartwin img {
    height: 16px;
    width: 16px;
}


.tableHead {
    display: flex;
    margin-left: 10px;
}

.tableHeaditem {
    flex: 1;
    padding: 10px;
    text-align: left;
    font-size: 12px;

}

.tableHeaditem:nth-child(3) {
    flex: 4;
}


.avgHead {
    /* height: 73vh; */
    /* background-color: rgb(86, 88, 98); */
    margin-left: 10px;
    margin-right: 10px;
    border-radius: 10px;
    display: flex;
    margin-left: 10px;
}


.avgHeaditem {
    flex: 1;
    padding: 10px;
    text-align: left;
    font-size: 12px;
}

.avgHeaditem:nth-child(3) {
    flex: 4;
}

.bodyname {
    display: flex;
}


.palyername {
    margin-left: 10px;
}

.duanwei {
    display: flex;
}

.duanwei img {
    height: 22px;
    width: 22px;
    margin-right: 5px;
}


.changwan {
    display: flex;
    justify-content: space-between;
}

.changwanname {
    color: yellow;
    margin-right: 5px;
}


.changwans {
    display: flex;
}

.wanjiatag {
    /* margin-left: 20px; */
    margin-right: 20px;
}


.avgranks {
    display: flex;
    justify-content: space-between;
}

.avgrank {
    display: flex;
    color: rgb(124, 92, 51);
    font-size: 14px;
    margin-left: 25px;
}

.avgbuju {
    display: flex;
    justify-content: center;
}

.showrank {
    color: white;
    font-size: 16px;
}
</style>