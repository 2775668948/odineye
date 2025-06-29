

// 获取当前玩家信息

// 从vuex中获取lcu连接信息
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { useStore } from 'vuex';

const store = useStore();

// 获取vuex中存储的lcu连接信息
export async function getLcuConnectionInfo() {
    try {
        // 从Vuex获取LCU连接信息
        const { token, port } = store.getters.getTokenAndPort;
        if (!token || !port) {
            throw new Error('LCU connection information is not available');
        }
        return { token, port };
    } catch (error) {
        console.error('从Vuex获取LCU连接信息失败:', error);
        throw error;
    }
}

// 从本地localStorage获取LCU连接信息
export function getLcuConnectionInfoFromLocalStorage() {

    const token = localStorage.getItem('lcuToken');
    const port = localStorage.getItem('lcuPort');
    if (!token || !port) {
        throw new Error('LCU connection information is not available in localStorage');
    }
    return { token, port };
}

// get方法获取数据
async function fetchLcuData(endpoint) {
    try {
        const data = await invoke('lcu_post_request', {
            endpoint
        });
        console.log("返回结果:", data);
        return data; // 别忘了返回数据
    } catch (error) {
        console.error("请求失败:", error);
        throw error;
    }
}

// 获取当前玩家信息
export async function getCurrentPlayer() {
    const endpoint = '/lol-summoner/v1/current-summoner';
    return await fetchLcuData(endpoint);
}

// 根据玩家summonerId获取rank战绩信息
export async function getRankedStatsByPuuid(summonerId) {
    const endpoint = `/lol-ranked/v1/ranked-stats/${summonerId}`;
    return await fetchLcuData(endpoint);
}

// 根据玩家puuid 获取段位信息
export async function getDivisionByPuuid(puuid) {
    const endpoint = `/lol-ranked/v1/ranked-stats/${puuid}`;
    return await fetchLcuData(endpoint);

}