export default {
    namespaced: true,
    state: () => ({
        token: '',
        port: 0
    }),
    mutations: {
        setTokenAndPort(state, payload) {
            state.token = payload.token;
            state.port = payload.port;
        }
    },
    getters: {
        getTokenAndPort: state => {
            return {
                token: state.token,
                port: state.port
            };
        }
    }
};