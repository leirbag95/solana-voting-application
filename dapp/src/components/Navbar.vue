<template>
  <div>
    <v-app-bar
      color="transparent"
      dense
      dark
      flat
    >
      <v-toolbar-title>Voting</v-toolbar-title>

      <v-spacer></v-spacer>

      <v-btn color="#4A148C">
        <v-icon left small>mdi-wallet</v-icon>
        Connect
      </v-btn>
    </v-app-bar>
  </div>
</template>

<script>

export default {
    // eslint-disable-next-line
    name: 'Navbar'
}
</script>