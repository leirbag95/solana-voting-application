<template>
  <v-container>
    <v-card class="my-5" outlined v-if="account">
      <v-card-title>Welcome</v-card-title>
      <v-stepper v-model="e1" class="transparent" >
        <v-stepper-header elevation="0">
          <v-stepper-step color="#4A148C" :complete="e1 > 1" step="1">
            Request Vote
          </v-stepper-step>

          <v-divider></v-divider>

          <v-stepper-step color="#4A148C" :complete="e1 > 2" step="2">
            Submit
          </v-stepper-step>
        </v-stepper-header>

        <v-stepper-items >
          <v-stepper-content step="1">
            <v-card class="mb-12" color="grey lighten-1" height="200px"></v-card>

            <v-btn color="#4A148C" @click="e1 = 2">
              Continue
            </v-btn>

            <v-btn text>
              Cancel
            </v-btn>
          </v-stepper-content>

          <v-stepper-content step="2">
            <v-card class="mb-12" color="grey lighten-1" height="200px"></v-card>

            <v-btn color="#4A148C" @click="e1 = 3">
              Continue
            </v-btn>

            <v-btn text>
              Cancel
            </v-btn>
          </v-stepper-content>
        </v-stepper-items>
      </v-stepper>
    </v-card>
    <HistoryTableVue/>
  </v-container>
</template>

<script>

import { mapState } from 'vuex'
import HistoryTableVue from '@/components/HistoryTable.vue';

export default {
  // eslint-disable-next-line
  name: 'Home',
  computed: mapState([
    'account',
    'provider'
  ]),
  components: {
    HistoryTableVue
  },
  data() {
    return {
      e1: 1
    };
  },
}
</script>
