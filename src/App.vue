<template>
  <Header></Header>
  <div class="app-main-wrapper">
    <div class="main-wrapper-menu"><Menu></Menu></div>
    <router-view></router-view>
  </div>
</template>

<script setup lang="ts">
import Header from "./components/header/header.vue";
import Menu from "./components/menu/menu.vue";
</script>

<style lang="scss" scoped>
.app-main-wrapper {
  width: 100%;
  height: calc(100vh - 60px);
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  flex-wrap: nowrap;
  overflow: hidden;
  .main-wrapper-menu {
    box-sizing: border-box;
    width: 60px;
    height: 100%;
    border-right: 1px solid #ff9c4b;
  }
}
</style>
