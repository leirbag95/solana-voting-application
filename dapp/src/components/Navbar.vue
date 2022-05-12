<template>
  <div>
    <v-app-bar
      color="transparent"
      dense
      dark
      flat
    >
      <v-toolbar-title>Voting System</v-toolbar-title>
      <v-spacer></v-spacer>
      
      <v-btn color="#4A148C" @click="connect()" v-if="!account">
        <v-icon left small>mdi-wallet</v-icon>
        Connect
      </v-btn>

      <v-chip
        v-else
        class="ma-2"
        color="white"
        outlined
      >
        <v-icon left>
           mdi-account-check
        </v-icon>
        {{account.publicKey.toString().slice(0, 4)}}...{{account.publicKey.toString().slice(-4)}}
      </v-chip>
    </v-app-bar>
  </div>
</template>

<script>
import { mapState } from 'vuex'


export default {
    // eslint-disable-next-line
    name: 'Navbar',
    computed: mapState([
    'account',
    'provider'
    ]),
    data () {
      return {
      }
    },
    methods: {
      connect () {
        this.$store.dispatch('connectProvider')
      }
    },
    modules: {
      // example
    },

    strict: false
}
</script>