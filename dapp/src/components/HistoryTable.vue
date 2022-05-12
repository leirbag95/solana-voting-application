<template>
  <v-card>
      <v-card-title>Historical data of the current vote</v-card-title>
    <v-card-title>
      <v-text-field
        v-model="search"
        append-icon="mdi-magnify"
        label="Search"
        single-line
        hide-details
      ></v-text-field>
    </v-card-title>
    <v-data-table
      :headers="headers"
      :items="desserts"
      :search="search"
    ></v-data-table>
  </v-card>
</template>

<script>
  export default {
    data () {
      return {
        search: '',
        headers: [
          {
            text: 'Address',
            align: 'start',
            filterable: false,
            value: 'address',
          },
          { text: 'Vote', value: 'vote' },
          { text: 'Datetime', value: 'datetime' }
        ],
        desserts: [
          {
            address: 'Afe4...3feb',
            vote: 'YES',
            datetime: '2022/01/01 13:23:56',
          }
        ],
      }
    },
  }
</script>