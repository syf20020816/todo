<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'form')">
      <el-form
        ref="userFormRef"
        :model="userForm"
        :rules="rules"
        label-width="120px"
        class="demo-userForm"
        :size="formSize"
        label-position="left"
        status-icon
      >
        <el-form-item label="username" prop="username">
          <el-input
            v-model="userForm.username"
            placeholder="please input your username"
          />
        </el-form-item>
        <el-form-item label="password" prop="password">
          <el-input
            v-model="userForm.password"
            show-password
            type="password"
            placeholder="please input your password"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm(userFormRef)"> Signin</el-button>
          <el-button type="warning" @click="toSignup"
            >No Account? Click to Signup</el-button
          >
        </el-form-item>
      </el-form>
    </div>
    <div :class="buildWrap(component, 'infos')">
      <el-alert
        title="username"
        type="info"
        description="you should input your username and the length should be 5 to 16"
        show-icon
      />
      <el-alert
        title="password"
        type="info"
        description="you should input the password and the length should be 5 to 16"
        show-icon
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  AvatarMap,
  Avatars,
  Teams,
  build,
  buildView,
  buildWrap,
  useTeam,
  UserLoginForm,
} from "../../../core";
import { ref, reactive } from "vue";
import api from "../../../api";
import { ElMessage } from "element-plus";

import type { FormInstance, FormRules } from "element-plus";
const component = "Signin";

const formSize = ref("default");
const userFormRef = ref<FormInstance>();
const userForm = reactive<UserLoginForm>({
  username: "",
  password: "",
});

// 类型“FormRules”不是泛型类型
// 无需理会这个错误（实际上并没有问题）
const rules = reactive<FormRules<UserLoginForm>>({
  username: [
    { required: true, message: "Please input your username", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
  password: [
    { required: true, message: "Please input your password", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
});

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate(async (valid, _fields) => {
    if (valid) {
      const data = await api.user.signin(userForm);
      console.log(data);
    } else {
      return;
    }
  });
};

const emits = defineEmits(["to-signup"]);

const toSignup = () => {
  emits("to-signup");
};
</script>

<style lang="scss" scoped>
@import "../../../styles/views/signin.scss";
</style>
