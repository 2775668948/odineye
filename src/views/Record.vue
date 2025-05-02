<template>
    <div class="outer-container" data-tauri-drag-region :class="{ 'hovered': isHovered }" @mouseenter="isHovered = true"
        @mouseleave="isHovered = false">
        <div class="content" data-tauri-drag-region>
            <!-- 内容区域也可以拖动 -->
            <div>
                <div class="title">
                    <span>奥丁之眼记牌器---认准官网 odineye.uk</span>
                </div>
                <div class="allyxs">
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_drmundo.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="蒙多">
                        <div class="corner-number">29</div>
                    </div>

                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_morgana.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="莫甘娜">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_shaco.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="萨科">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_zyra.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="婕拉">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_nidaleecougar.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="奈德丽">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_kindred.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="千珏">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_seraphine.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="萨勒芬妮">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_kogmaw.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="克格莫">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_jax.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="贾克斯">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_poppy.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="波比">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_sylas.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="塞拉斯">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_vi.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="蔚">
                        <div class="corner-number">30</div>
                    </div>
                    <div class="image-container feika1">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_alistar.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="阿利斯塔">
                        <div class="corner-number">30</div>
                    </div>

                </div>

                <div class="allyxs">
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_darius.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="德莱厄斯">
                        <div class="corner-number">25</div>
                    </div>

                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_leblanc.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="乐芙兰">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_twistedfate.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="崔斯特">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_veigar.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="维迦">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_shyvana.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="希瓦娜">
                        <div class="corner-number ">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_illaoi.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="俄洛伊">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_jhin.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="烬">
                        <div class="corner-number ">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_naafiri.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="纳亚菲利">
                        <div class="corner-number ">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_skarner.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="斯卡纳">
                        <div class="corner-number ">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_vayne.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="VN">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_graves.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="格雷福斯">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_rhaast.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="拉亚斯特">
                        <div class="corner-number">25</div>
                    </div>
                    <div class="image-container feika2">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_ekko.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="艾克">
                        <div class="corner-number">25</div>
                    </div>

                </div>

                <div class="allyxs">
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_elise.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="伊丽丝">
                        <div class="corner-number">18</div>
                    </div>

                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_fiddlesticks.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="费德提克">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_galio.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="加里奥">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_rengar.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="雷恩加尔">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_senna.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="赛娜">
                        <div class="corner-number ">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_varus.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="韦鲁斯">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_braum.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="布隆">
                        <div class="corner-number ">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_yuumi.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="悠米">
                        <div class="corner-number ">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_gragas.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="古拉加斯">
                        <div class="corner-number ">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_mordekaiser.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="莫德凯撒">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_jarvan.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="嘉文四世">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_draven.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="德莱文">
                        <div class="corner-number">18</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_jinx.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="金克丝">
                        <div class="corner-number">18</div>
                    </div>

                </div>

                <div class="allyxs">
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_brand.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="布兰德">
                        <div class="corner-number">10</div>
                    </div>

                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_missfortune.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="厄运小姐">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_neeko.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="妮蔻">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_vex.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="薇古丝">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_zed.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="劫">
                        <div class="corner-number ">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_zeri.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="泽丽">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_xayah.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="霞">
                        <div class="corner-number ">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_sejuani.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="瑟庄妮">
                        <div class="corner-number ">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_ziggs.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="吉格斯">
                        <div class="corner-number ">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_chogath.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="科加斯">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_leona.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="蕾欧娜">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika3">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_annie.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="安妮">
                        <div class="corner-number">10</div>
                    </div>
                    <div class="image-container feika4">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_aphelios.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="厄斐琉斯">
                        <div class="corner-number">10</div>
                    </div>

                </div>

                <div class="allyxs">
                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_garen.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="盖伦">
                        <div class="corner-number">9</div>
                    </div>

                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_renekton.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="雷克顿">
                        <div class="corner-number">9</div>
                    </div>
                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_samira.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="莎弥拉">
                        <div class="corner-number">9</div>
                    </div>
                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_kobuko.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="可酷伯">
                        <div class="corner-number">9</div>
                    </div>
                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_urgot.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="厄加特">
                        <div class="corner-number ">9</div>
                    </div>
                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_aurora.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="阿罗拉">
                        <div class="corner-number">9</div>
                    </div>
                    <div class="image-container feika5">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_viego.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="佛耶戈">
                        <div class="corner-number ">9</div>
                    </div>
                    <div :class="[isClicked ? 'xuanzhong' : 'feika5', 'image-container']">
                        <img src="https://s-tft-api.op.gg/img/set/14/tft-champion/tiles/tft14_zac.tft_set14.png?image=q_auto:good,f_webp&b=YYWBQnfWUKIR1zzRMhzVG"
                            alt="扎克" @click="toggleBorder">
                        <div class="corner-number ">9</div>
                    </div>


                </div>
            </div>

            <button @click="showMainWindow">111</button>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue'

import { invoke } from "@tauri-apps/api/core";
const isHovered = ref(false)

const AllCards = ref([
])
async function showMainWindow() {
    try {
        await invoke('showgmwindow');  // 调用后端命令展示指定窗口
        console.log('已展示窗口');
    } catch (error) {
        console.error('展示窗口失败:', error);
    }
}

const isClicked = ref(false)

function toggleBorder() {
    isClicked.value = !isClicked.value
}


</script>

<style scoped>
.outer-container {
    width: 100vw;
    height: 100vh;
    border-radius: 20px;
    background: linear-gradient(to bottom,
            rgba(30, 30, 30, 0.8),
            rgba(30, 30, 30, 0.3));
    overflow: hidden;
    -webkit-app-region: drag;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    transition: all 0.3s ease;
}

.outer-container.hovered {
    background: linear-gradient(to bottom,
            rgba(50, 50, 50, 0.85),
            rgba(50, 50, 50, 0.4));
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

.title {
    height: 60px;
    width: 100vw;
    background-color: black;
    display: flex;
    justify-content: center;
    align-items: center;
    /* ⭐️ 垂直居中 */

}

.allyxs {
    margin-top: 5px;
    margin-left: 10px;

}

.image-container {
    position: relative;
    display: inline-block;
    margin-right: 5px;
}

.image-container img {
    display: block;
    width: 36px;
    /* 你可以根据需要调整大小 */
    border-radius: 4px;
    /* 可选，加一点圆角美观 */

    /* 重点：加了绿色边框 */

}


.feika1 {
    border: 2px solid rgb(75, 76, 80);
    border-radius: 4px;
}

.feika2 {
    border: 2px solid rgb(0, 255, 85);
    border-radius: 4px;
}

.feika3 {
    border: 2px solid rgb(0, 168, 255);
    border-radius: 4px;
}

.feika4 {
    border: 2px solid rgb(255, 0, 221);
    border-radius: 4px;
}


.feika5 {
    border: 2px solid rgb(247, 192, 13);
    border-radius: 4px;
}

.corner-number {
    position: absolute;
    left: -4px;
    bottom: 1px;
    /* 半透明黑色背景 */
    color: white;
    padding: 1px 8px;
    font-size: 11px;
    border-radius: 4px;
    font-weight: bolder;
}


/* 阵容选中的牌 */
.xuanzhong {
    border: 2px solid rgb(226, 41, 41);
    border-radius: 4px;
}
</style>