<template>
    <div class="main-con">

        <n-infinite-scroll>
            <div v-for="(team, index) in teamList" :key="index" class="champion">
                <div class="container-con">
                    <div class="container">
                        <div class="rank-badge">
                            <svg t="1745656173446" class="icon" viewBox="0 0 1024 1024" version="1.1"
                                xmlns="http://www.w3.org/2000/svg" p-id="1849" width="200" height="200">
                                <path
                                    d="M522.965333 319.573333a160.853333 160.853333 0 0 1 99.84 27.136 114.901333 114.901333 0 0 1 43.52 88.064h-55.466666a79.786667 79.786667 0 0 0-28.672-51.2 102.741333 102.741333 0 0 0-62.464-15.872 115.925333 115.925333 0 0 0-55.466667 10.752 41.770667 41.770667 0 0 0-23.552 39.936 43.349333 43.349333 0 0 0 27.136 38.912 377.088 377.088 0 0 0 63.488 22.528 537.130667 537.130667 0 0 1 95.744 35.84 88.32 88.32 0 0 1 46.933333 78.848 91.52 91.52 0 0 1-38.4 77.312 181.504 181.504 0 0 1-107.52 27.648 180.608 180.608 0 0 1-103.936-26.624 127.36 127.36 0 0 1-49.664-101.376h55.466667a89.045333 89.045333 0 0 0 30.72 61.952 113.066667 113.066667 0 0 0 68.266667 17.066667 134.314667 134.314667 0 0 0 65.706666-14.506667 42.666667 42.666667 0 0 0 24.576-38.4 53.930667 53.930667 0 0 0-34.816-48.128q-12.8-6.144-72.533333-24.576a719.530667 719.530667 0 0 1-83.968-30.208 80.384 80.384 0 0 1-42.154667-73.216 87.04 87.04 0 0 1 40.448-75.776 168.362667 168.362667 0 0 1 96.768-26.112z"
                                    fill="#F6BD16" p-id="1850"></path>
                            </svg>
                        </div>
                        <div class="info">
                            <div class="title">{{ team.title }}</div>
                            <div class="champions">

                                <div v-for="(champion, championindex) in team.champions" :key="championindex"
                                    class="champion">

                                    <img :src="champion.src" alt="Champion 1">
                                    <div class="items" v-if="champion.haveEquipment">
                                        <div v-for="(equipment, equipmentindex) in champion.Equipment"
                                            :key="equipmentindex">
                                            <div class="item"><img :src="equipment.equipmentSrc" alt="Item 1"></div>
                                        </div>

                                    </div>
                                </div>


                            </div>
                        </div>
                        <div class="stats">
                            <div class="stat">
                                <div class="value">{{ team.stats.avgRank }}</div>
                                <div>平均名次</div>
                            </div>
                            <div class="stat">
                                <div class="value">{{ team.stats.winRate }}</div>
                                <div>第一名</div>
                            </div>
                            <div class="stat">
                                <div class="value">{{ team.stats.top4Rate }}</div>
                                <div>前四率</div>
                            </div>
                            <div class="stat">
                                <div class="value">{{ team.stats.pickRate }}</div>
                                <div>挑选率</div>
                            </div>
                        </div>
                    </div>

                </div>

            </div>
        </n-infinite-scroll>

    </div>
</template>


<script setup>



const teamList = [
    {
        title: "源计划 薇古丝",
        champions: [
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_jax.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "贾克斯",
                "haveEquipment": false,
                "Equipment": [
                    {
                        "equipmentSrc": ""
                    },
                    {
                        "equipmentSrc": ""
                    },
                    {
                        "equipmentSrc": ""
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_morgana.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "莫甘娜",
                "haveEquipment": false,
                "Equipment": [
                    {
                        "equipmentSrc": ""
                    },
                    {
                        "equipmentSrc": ""
                    },
                    {
                        "equipmentSrc": ""
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_jhin.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "烬",
                "haveEquipment": false,
                "Equipment": [
                    {
                        "equipmentSrc": ""
                    },
                    {
                        "equipmentSrc": ""
                    },
                    {
                        "equipmentSrc": ""
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_gragas.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "古拉加斯",
                "haveEquipment": false,
                "Equipment": [
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_mordekaiser.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "莫德凯撒",
                "haveEquipment": true,
                "Equipment": [
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT14_MordekaiserCyberneticItem.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_Crownguard.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_NightHarvester.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_varus.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "维鲁斯",
                "haveEquipment": true,
                "Equipment": [
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT14_NaafiriCyberneticItem.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_RapidFireCannon.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_StatikkShiv.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_sejuani.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "瑟庄妮",
                "haveEquipment": true,
                "Equipment": [
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_WarmogsArmor.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_FrozenHeart.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_Redemption.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_vex.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "薇古丝",
                "haveEquipment": true,
                "Equipment": [
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_BlueBuff.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_HextechGunblade.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    },
                    {
                        "equipmentSrc": "https://s-tft-api.op.gg/img/set/14/tft-item/TFT_Item_PowerGauntlet.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                    }
                ]
            },
            {
                "src": "https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_aurora.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG",
                "name": "阿萝拉",
                "haveEquipment": false,
                "Equipment": [
                ]
            }

        ],
        stats: {
            pickRate: "2.07%",
            avgRank: "3.06",
            top4Rate: "79.39%",
            winRate: "21.40%"
        }
    }
]

const callRecord = () => {
    console.log("打开记牌器....")


}

</script>

<style scoped>
.main-con {

    height: 73vh;
    margin-top: 7vh;
    border-radius: 2vh;
    margin-left: 1vh;
}

.container-con {
    background: rgb(18, 23, 35);
    font-family: Arial, sans-serif;
    color: #c5f6fa;
    margin: 0;
    padding: 20px;
}

.container {
    background: rgb(10, 12, 22);
    border-radius: 12px;
    padding: 20px;
    display: flex;
    align-items: center;
    gap: 20px;
    /* margin-top: 20px; */
}

.rank-badge {
    padding: 10px;
    border-radius: 8px;
    font-weight: bold;
    color: #333;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 50px;
    height: 50px;
    margin-top: 35px;
}

.info {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.title {
    font-size: 20px;
    font-weight: bold;
}

.champions {
    display: flex;
    gap: 10px;
    margin-top: 10px;
}

.champion {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.champion img {
    width: 50px;
    height: 50px;
    border-radius: 8px;
}

.star {
    position: absolute;
    top: -5px;
    left: -5px;
    background: #fff;
    color: #000;
    font-size: 12px;
    border-radius: 50%;
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
}

.items {
    display: flex;
    gap: 2px;
    margin-top: -10px;
}

.item img {
    width: 15px;
    height: 15px;
    border-radius: 4px;
    margin: 1px;
}

.stats {
    display: flex;
    gap: 20px;
    margin-left: auto;
    text-align: center;
    font-size: 10px;
    margin-top: 35px;
}

.stat {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.value {
    font-size: 18px;
    font-weight: bold;
}
</style>