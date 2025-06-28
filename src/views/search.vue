<template>
    <div class="search-container">
        <div class="left-content">
            <div class="search-content">
                <!-- 搜索框和搜索按钮 -->
                <div class="top">
                    <!-- 搜索框 -->
                    <div class="search-input-outer">
                        <div class="search-input-inner">
                            <input type="text" v-model="palayerName" placeholder="召唤师名称" class="search-input" />
                        </div>
                    </div>
                    <!-- 搜索按钮 -->
                    <div class="search-button" @click="handleSearchPlayer">
                        <span>查询</span>
                    </div>
                </div>
                <!-- 中间的段位 -->
                <div class="search-duanwei">
                    <!-- 左边的单双段位 -->
                    <div class="search-duanwei-left">
                        <span>单: 坚韧黑铁1(26)</span>
                    </div>
                    <div class="search-duanwei-right">
                        <span>灵: 坚韧黑铁3(90)</span>
                    </div>
                </div>
                <!-- 下面的最近常玩 -->
                <div class="likeplay">
                    <div class="likeplaytitle">
                        <span>最近常玩:</span>
                    </div>
                    <div class="likeplayhero">
                        <img src="https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png"
                            alt="" class="likeplayheroimg" />
                        <img src="https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png" alt=""
                            class="likeplayheroimg" />
                        <img src="https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png" alt=""
                            class="likeplayheroimg" />
                    </div>
                </div>
            </div>
            <div class="res-content">
                <n-infinite-scroll :distance="10" :loading="false" :show-loading="false">
                    <div v-for="(item, index) in data" :key="index" class="res-item-container">
                        <!-- 动态点击切换 -->
                        <div class="res-item" :class="{
                            'res-item-win': item.iswin,
                            'res-item-lose': !item.iswin,
                            'clicked-class-win': activeIndex === index && item.iswin,
                            'clicked-class-lose': activeIndex === index & active == true
                        }" @click="handleClick(index)">
                            <!-- 英雄头像 -->
                            <div>
                                <img :src="item.heroimg" alt="" class="res-item-playimg"
                                    :class="{ 'res-item-playimg-win': item.iswin, 'res-item-playimg-lose': !item.iswin }" />
                            </div>
                            <!-- 排位模式 -->
                            <div class="res-item-mode">
                                <span>{{ item.mode }}</span>
                            </div>
                            <!-- 游戏战绩 -->
                            <div class="res-item-result"
                                :class="{ 'res-item-result-win': item.iswin, 'res-item-result-lose': !item.iswin }">
                                <span>{{ item.result }}</span>
                            </div>
                            <!-- 游戏时间 -->
                            <div class="res-item-time">
                                <span>{{ item.time }}</span>
                            </div>
                        </div>
                    </div>

                </n-infinite-scroll>
            </div>
        </div>
        <!-- 右边的对局详情 -->
        <div class="right-content">
            <!-- 头部的几个标识 -->
            <div class="right-top-content">
                <div class="top-content-hero">
                    <span>英雄</span>
                </div>
                <div class="top-content-nickname">
                    <span>召唤师名称</span>
                </div>
                <div class="top-content-KDA">
                    <span>KDA</span>
                </div>
                <!-- 召唤师技能 -->
                <div class="top-content-skill">
                    <span>召唤师技能</span>
                </div>
                <!-- 出装 -->
                <div class="top-content-build">
                    <span>出装</span>
                </div>
            </div>
            <!-- 然后是剩下的对局详情 -->
            <div class="game-content">
                <!-- 上面是输的一方 -->
                <div class="losers-game">
                    <!-- 然后就是具体的5个玩家 -->
                    <div class="loser-player-item" v-for="(item, index) in losersGameDeatail" :key="index">
                        <!-- 英雄头像 -->
                        <div>
                            <img :src="item.heroimg" alt="" class="loser-player-heroimg" />
                        </div>
                        <!-- 召唤师名称 -->
                        <div class="loser-player-nickname-item">
                            <div>
                                <span>{{ item.nickname }}</span>
                            </div>
                            <!-- 复制昵称 -->
                            <div class="loser-player-nickname-copy" @click="handleCopyNickName(item.nickname)">
                                <img src="../assets/copy.png" alt="" class="loser-player-nickname-copy-img" />
                            </div>
                        </div>
                        <!-- 输的人的KDA -->
                        <div class="loser-player-KDA">
                            <span>{{ item.KDA }}</span>
                        </div>
                        <!-- 输的人带的召唤师技能 -->
                        <div class="loser-player-skill">

                            <div>
                                <img v-for="(skill, skillIndex) in item.skill" :key="skillIndex" :src="skill" alt=""
                                    class="loser-player-skill-img" />
                            </div>
                        </div>
                        <!-- 输的人出的装备 -->
                        <div class="loser-player-build">
                            <div v-for="(build, buildIndex) in item.build" :key="buildIndex">
                                <img :src="build" alt="" class="loser-player-build-img" />
                            </div>
                        </div>
                    </div>
                </div>










                <!-- 下面是赢的一方 -->
                <div class="winer-game">
                    <div class="loser-player-item" v-for="(item, index) in winnersGameDeatail" :key="index">
                        <!-- 英雄头像 -->
                        <div>
                            <img :src="item.heroimg" alt="" class="loser-player-heroimg-win" />
                        </div>
                        <!-- 召唤师名称 -->
                        <div class="loser-player-nickname-item">
                            <div>
                                <span>{{ item.nickname }}</span>
                            </div>
                            <!-- 复制昵称 -->
                            <div class="loser-player-nickname-copy" @click="handleCopyNickName(item.nickname)">
                                <img src="../assets/copy.png" alt="" class="loser-player-nickname-copy-img" />
                            </div>
                        </div>
                        <!-- 赢的人的KDA -->
                        <div class="loser-player-KDA-win">
                            <span>{{ item.KDA }}</span>
                        </div>
                        <!-- 赢的人带的召唤师技能 -->
                        <div class="loser-player-skill-win">
                            <div>
                                <img v-for="(skill, skillIndex) in item.skill" :key="skillIndex" :src="skill" alt=""
                                    class="loser-player-skill-img" />
                            </div>
                        </div>
                        <!-- 赢的人出的装备 -->
                        <div class="loser-player-build">
                            <div v-for="(build, buildIndex) in item.build" :key="buildIndex">
                                <img :src="build" alt="" class="loser-player-build-img" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup>
import { ref, computed } from 'vue';
const palayerName = ref('');


// 模拟的战绩数据
const data = ref([
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 1001 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 1002 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 1003 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 1004 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 1006 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 1011 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 112301 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 1412 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 14121 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 4123 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 15421 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 12121 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 512412 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 123421 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 51231231 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 52312 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 5532 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 53276 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 852332 },
    { mode: '灵活排位', result: '10/5/15', time: '06-01 14:30', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png", iswin: true, "id": 52174 },
    { mode: '单双排位', result: '8/2/12', time: '05-31 18:20', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png", iswin: true, "id": 34523 },
    { mode: '单双排位', result: '6/13/9', time: '06-02 09:44', heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Aphelios.png", iswin: false, "id": 5322232 },
]);


// 当前点击是第几个
const currentIndex = ref(0);
// 处理点击事件
const handleClick = (index) => {
    currentIndex.value = index;
    console.log(activeIndex.value);
    console.log("当前点击的是第" + index + "个");
    activeIndex.value = index; // 设置当前激活的索引
    // 判断是赢了还是输了
    if (data.value[index].iswin) {
        console.log("赢了");
        // 动态添加CSS类名

        active.value = true; // 设置active为true
    } else {
        console.log("输了");
        active.value = true;
    }
};




// 处理动态点击CSS类名
const active = ref(false);

const activeIndex = ref(0);


// 模拟输的5个玩家的游戏详情
const losersGameDeatail = ref([
    {
        pleyerId: 1,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称1",
        KDA: "6/13/9",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png",
        ]
    },
    {
        pleyerId: 2,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png",
        nickname: "召唤师名称2",
        KDA: "10/5/15",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 3,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称3",
        KDA: "8/2/12",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 4,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称4",
        KDA: "6/13/9",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 5,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称5",
        KDA: "6/13/9",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    }

]);


// 模拟赢的5个玩家的游戏详情
const winnersGameDeatail = ref([

    {
        pleyerId: 1,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称6",
        KDA: "6/13/9",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 2,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Nilah.png",
        nickname: "召唤师名称7",
        KDA: "10/5/15",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 3,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称8",
        KDA: "8/2/12",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 4,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称9",
        KDA: "6/13/9",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    },
    {
        pleyerId: 5,
        heroimg: "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/champion/Kaisa.png",
        nickname: "召唤师名称10",
        KDA: "6/13/9",
        skill: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerFlash.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/spell/SummonerHaste.png"
        ],
        build: [
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6673.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6672.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6671.png",
            "https://opgg-static.akamaized.net/meta/images/lol/15.13.1/item/6670.png"
        ]
    }
]);


// 点击复制召唤师名称
const handleCopyNickName = (nickname) => {
    navigator.clipboard.writeText(nickname).then(() => {
        console.log('复制成功:', nickname);
    }).catch(err => {
        console.error('复制失败:', err);
    });
};


// 处理搜索按钮点击事件
const handleSearchPlayer = () => {
    console.log('搜索 button clicked!');
    // 这里可以添加搜索逻辑
    // 比如调用API获取数据等
    // 目前只是打印选中的模式
    console.log('palayerName:', palayerName.value);
    // 如果是空的，那么就是查询当前玩家的战绩
    if (palayerName.value.trim() === '') {
        console.log('查询当前玩家的战绩');
        // 这里可以添加查询当前玩家战绩的逻辑
        getCurrentPlayerStats();
    } else {
        console.log('查询其他玩家的战绩:', palayerName.value);
        // 这里可以添加查询其他玩家战绩的逻辑
    }
};

// 获取当前玩家的战绩
const getCurrentPlayerStats = () => {
    // 这里可以添加获取当前玩家战绩的逻辑
    console.log('从Lcu获取当前玩家的战绩...');
};
</script>


<style scoped>
.search-container {
    display: flex;
    height: 550px;
    /* background-color: red; */
    margin-top: 15px;
}


.left-content {
    width: 270px;
    margin-right: 10px;
    border-radius: 10px;
}

.search-content {
    height: 110px;
    background-color: #121723;
    border-radius: 5px;
}

.top {
    /* background-color: red; */
    /* height: 45px; */
    display: flex;
    padding-top: 10px;
    padding-left: 10px;
    padding-bottom: 0;
    margin-bottom: 0px;
}

.search-button {
    width: 70px;
    height: 30px;
    background-color: #01E1EE;
    color: #214939;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    margin-left: 10px;
    transition: background-color 0.3s ease;
}

.search-input-outer {
    height: 30px;
    width: 140px;
    background-color: #101323;
    margin-right: 10px;
    border-radius: 5px;
    /* 一个0.5px的实线 */
    border: 0.5px solid #23243a;
}


.search-input-inner {
    margin-left: 10px;
    margin-top: 2px;
    width: 100px;
}

.search-input {
    margin-top: 2px;
    width: 100%;
    height: 20px;
    background-color: #101323;
    color: #8D8D8D;
    border: none;
    outline: none;
    font-size: 12px;
}

/* input框点击的样式 */
.search-input:focus {
    color: white;
}


.search-duanwei {
    /* background-color: red; */
    height: 20px;
    margin-left: 10px;
    display: flex;
    margin-top: 5px;
    color: #8D8D8D;
    font-size: 11px;
}

.search-duanwei-right {
    margin-left: 40px;
}

.likeplay {
    display: flex;
    justify-content: space-between;
    margin-left: 10px;
    color: #8D8D8D;
    font-size: 11px;
    margin-top: 5px;
    align-items: center;
}

.likeplayhero {
    display: flex;
    margin-right: 5px;
}

/* 图片是一个圆形 */
.likeplayheroimg {
    width: 25px;
    height: 25px;
    border-radius: 10px;
    margin-right: 5px;
}

.res-content {
    margin-top: 10px;
    height: 430px;
    background-color: #121723;
    border-radius: 5px;
}

.res-item {
    height: 50px;
    display: flex;
    align-items: center;
    /* 实现一个绿到淡淡绿的渐变 中间更分明一些*/
    background: linear-gradient(to right, #102A2F, #121723);
    /* border-radius: 5px; */
    background-color: #01C791;
    justify-content: space-between;
}

/* 输游戏的渐变 */
.res-item-lose {
    background: linear-gradient(to right, #2A1F1F, #121723);
}

/* 输了游戏鼠标放上去的渐变 */
.res-item-lose:hover {
    background: linear-gradient(to right, #7B3238, #121723);
    cursor: pointer;
}

/* 输了游戏点击上去也要渐变 */
.clicked-class-lose {
    background: linear-gradient(to right, #7B3238, #121723);
    cursor: pointer;
}

/* res-item 鼠标放上去的效果 */
.res-item-win:hover {
    background: linear-gradient(to right, #276658, #11252C);
    cursor: pointer;
}

/* 点击了赢游戏也要切换背景 */
.clicked-class-win {
    background: linear-gradient(to right, #276658, #11252C);
    cursor: pointer;
}

.res-item-playimg {
    width: 30px;
    height: 30px;
    border-radius: 20px;
    margin-left: 10px;
    margin-top: 10px;
    /* 图片上套一个1px的border */
    border: 2px solid #064634;
}

/* 输游戏的图片边框 */
.res-item-playimg-lose {
    border: 2px solid #FA5151;
}

.res-item-mode {
    margin-left: 10px;
    color: white;
    font-size: 12px;
    font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
    margin-top: 8px;
}

.res-item-result-win {
    margin-left: 15px;
    margin-top: 5px;
    color: #0BD293;
    font-size: 12px;
}

/* 输了游戏的战绩文字 */
.res-item-result-lose {
    margin-left: 15px;
    margin-top: 5px;
    color: #FA5151;
    font-size: 12px;
}

/*  */


.res-item-time {
    margin-left: 15px;
    margin-top: 5px;
    color: #8D8D8D;
    font-size: 12px;
}

.res-item-container {
    margin-right: 20px;
}


.right-content {
    height: 550px;
    width: 770px;
    background-color: #121723;
    border-radius: 10px;
}

.right-top-content {
    height: 40px;
    display: flex;
    align-items: center;
    color: #8D8D8D;
}

.top-content-hero {
    width: 80px;
    display: flex;
    justify-content: center;
}

.top-content-nickname {
    width: 150px;
    display: flex;
    justify-content: center;
}

.top-content-KDA {
    width: 80px;
    display: flex;
    justify-content: center;
}

.top-content-skill {
    width: 100px;
    display: flex;
    justify-content: center;
    margin-left: 20px;
}

.top-content-build {
    width: 200px;
    display: flex;
    /* justify-content: center; */
    margin-left: 20px;
}

.game-content {
    height: 500px;
    display: flex;
    flex-direction: column;
    /* 从上到下等分 中间有个30px间隔*/
}

.losers-game {
    height: 300px;
    background: linear-gradient(to right, #291D28, #121723);
}

.winer-game {
    height: 310px;
    background: linear-gradient(to right, #102A2F, #121723);
    margin-top: 33px;
    padding-bottom: 0;
}

/* 输的每一个人 */
.loser-player-item {
    height: 46px;
    display: flex;
    align-items: center;

}

/* 输的人玩的英雄 */
.loser-player-heroimg {
    width: 30px;
    height: 30px;
    border-radius: 20px;
    margin-left: 25px;
    margin-top: 10px;
    /* 图片上套一个2px的border */
    border: 2px solid #FA5151;
}

/* 赢的人玩的英雄 */
.loser-player-heroimg-win {
    border: 2px solid #0BD293;
    margin-left: 25px;
    margin-top: 10px;
    width: 30px;
    height: 30px;
    border-radius: 20px;
}

/* 输得人的名称 */
.loser-player-nickname {}

.loser-player-nickname-item {
    margin-left: 63px;
    display: flex;
    justify-content: center;
    color: #8D8D8D;
    font-size: 12px;
    margin-top: 3px;
}

/* 复制昵称 */
.loser-player-nickname-copy-img {
    width: 12px;
    height: 12px;
    cursor: pointer;
    margin-top: 3px;
    margin-left: 2px;
}


/* 输的人的KDA */
.loser-player-KDA {
    margin-left: 50px;
    color: #FA5151;
    font-size: 14px;
    margin-top: 5px;
    display: flex;
    justify-content: start;
    margin-right: 2px;
    width: 60px;
}

.loser-player-KDA span {
    padding-right: 10px;
}

/* 赢的人的KDA */
.loser-player-KDA-win {
    margin-left: 50px;
    color: #0BD293;
    font-size: 14px;
    margin-top: 5px;
    display: flex;
    justify-content: start;
    margin-right: 2px;
    width: 60px;
}

/* 输的人带的召唤师技能 */
.loser-player-skill {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-left: 28px;
}

/* 输的人带的召唤师技能图片 */
.loser-player-skill-img {
    width: 20px;
    height: 20px;
    margin-left: 10px;
    margin-top: 4px;
    border-radius: 5px;
}

/* 输的人带的装备 */
.loser-player-build {
    weight: 100px;
    display: flex;
    align-items: center;
    margin-left: 40px;
    justify-content: flex-start;
}

/* 输的人带的装备图片 */
.loser-player-build-img {
    width: 20px;
    height: 20px;
    margin-left: 10px;
    margin-top: 5px;
    border-radius: 5px;
}

/* 赢的人带的召唤师技能 */
.loser-player-skill-win {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-left: 28px;
}
</style>
