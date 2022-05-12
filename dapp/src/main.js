import Vue from 'vue'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'
import LottieAnimation from "lottie-vuejs/src/LottieAnimation.vue";
import store from "./store/index.js";

Vue.use(LottieAnimation); 

Vue.config.productionTip = false

Vue.prototype.$store = store;

new Vue({
  router,
  vuetify,
  store,
  render: function (h) { return h(App) }
}).$mount('#app')
