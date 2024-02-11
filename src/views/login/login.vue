<template>
  <div :id="buildView(component)">
    <el-tabs tab-position="left" style="height: 100%" v-model="tabActive">
      <el-tab-pane label="Signin" name="signin">
        <Signin @to-signup="toSignup"></Signin>
      </el-tab-pane>
      <el-tab-pane label="Signup" name="signup">
        <Signup @to-signin="toSignin"></Signup>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script lang="ts" setup>
import {
  AvatarMap,
  Avatars,
  TeamAvatars,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../../core";
import { ref, reactive } from "vue";
import Signin from "./components/signin.vue";
import Signup from "./components/signup.vue";
const component = "Login";

const tabActive = ref("signin");

const toSignup = () => {
  tabActive.value = "signup";
};

const toSignin = () => {
  tabActive.value = "signin";
};
</script>

<style lang="scss" scoped>
@import "../../styles/views/login.scss";
</style>
