<script setup lang="ts">
import GyroscopeDisplay from "./components/GyroscopeDisplay.vue";
</script>

<template>
  <div class="app">
    <h1>手機陀螺儀測試</h1>
    <GyroscopeDisplay />
  </div>
</template>

<style scoped>
.app {
  padding: 20px;
  text-align: center;
}

h1 {
  color: #2c3e50;
  margin-bottom: 30px;
}
</style>
