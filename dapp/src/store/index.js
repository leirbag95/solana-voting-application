import Vue from 'vue'
import Vuex from 'vuex'

import { getProvider } from '@/services/phantom'

Vue.use(Vuex)

export default new Vuex.Store({
    state: {
        provider: null,
        account: null
    },
    mutations: {
        setAccount (state, account) {
            state.account = account
        },

        setProvider (state, provider) {
            state.provider = provider
        }
    },

    actions: {
        async connectProvider ({commit}) {
            let provider = getProvider()
            if (provider) {
                await commit ('setProvider', getProvider())
                try {
                    const resp = await window.solana.connect();
                    await commit ('setAccount', resp)
                } catch (err) {
                    // { code: 4001, message: 'User rejected the request.' }
                    console.log("an error has occured", err)
                }
            }
        }
    }
})