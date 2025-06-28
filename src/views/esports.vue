<template lang="">
    <div>
        这是电子竞技界面
    </div>
</template>
<script setup>
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from 'vuex';

const store = useStore();

onMounted(() => {
    // 这里可以执行一些初始化操作
    console.log('电子竞技界面已挂载');
    // 设置延时定时器5s后再获取
    setTimeout(() => {
        console.log('5秒后获取port和token');
        getTokenAndPort();
    }, 5000);

});

// 向rust拿取port和token
const getTokenAndPort = async () => {
    try {
        const [token, port] = await invoke("return_port_and_token");
        console.log(`Port: ${port}, Token: ${token}`);
        store.commit('lcu/setTokenAndPort', { token, port });
    } catch (error) {
        console.error('获取port和token失败:', error);
    }
};


</script>
<style lang="">

</style>