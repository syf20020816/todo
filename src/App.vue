<template>
  <Header></Header>
  <div class="app-main-wrapper" v-if="userStore.isSignIn">
    <div class="main-wrapper-menu"><Menu></Menu></div>
    <router-view></router-view>
  </div>
  <div v-else class="app-main-wrapper">
    <Login></Login>
  </div>
</template>

<script setup lang="ts">
import Header from "./components/header/header.vue";
import Menu from "./components/menu/menu.vue";
import { user as userPinia } from "./store/src/user";
import { init } from "./core";
import { onMounted } from "vue";
import Login from "./views/login/login.vue";
import api from "./api";

const userStore = userPinia();

userStore.checkSetIsSignIn();

const initApp = async () => {
  let username = userStore.getUsername();
  if (typeof username !== "undefined") {
    const data = await api.user.getUserInfo(username.toString());

    if (typeof data !== "undefined") {
      userStore.setUser(data);
      userStore.setSignIn();
    }
  }
};

onMounted(() => {
  initApp();
});
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
