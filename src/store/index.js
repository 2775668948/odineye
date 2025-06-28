import { createStore } from 'vuex';
import lcu from './lcu';

const store = createStore({
    modules: {
        lcu
    }
});

export default store;
