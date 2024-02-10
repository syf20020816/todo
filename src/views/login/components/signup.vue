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
        <el-form-item label="name" prop="name">
          <el-input v-model="userForm.name" placeholder="please input your name" />
        </el-form-item>
        <el-form-item label="email" prop="email">
          <el-input v-model="userForm.email" placeholder="please input your email" />
        </el-form-item>
        <el-form-item label="password" prop="password">
          <el-input
            v-model="userForm.password"
            show-password
            type="password"
            placeholder="please input your password"
          />
        </el-form-item>
        <el-form-item label="check pwd" prop="pwdCheck">
          <el-input
            show-password
            type="password"
            v-model="userForm.pwdCheck"
            placeholder="please input your password again"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm(userFormRef)">
            Create Account</el-button
          >
          <el-button @click="resetForm(userFormRef)">Reset All</el-button>
        </el-form-item>
      </el-form>
    </div>
    <div :class="buildWrap(component, 'infos')">
      <el-alert
        title="About Signup"
        type="warning"
        description="If you wanna use MYTODO, you should signup first"
        show-icon
      />
      <el-alert
        title="username"
        type="info"
        description="you should have a username and the length should be 5 to 16"
        show-icon
      />
      <el-alert
        title="name"
        type="info"
        description="you should input your name, recommend to use your real name"
        show-icon
      />
      <el-alert
        title="email"
        type="info"
        description="you should input your email, it will be used for work"
        show-icon
      />
      <el-button type="success" @click="toSignin">back to signin</el-button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  buildView,
  buildWrap,
} from "../../../core";
import { ref, reactive } from "vue";

import { ElMessage, type FormInstance, type FormRules } from "element-plus";
import api from "../../../api";
import {user as userPinia} from '../../../store/src/user'


const component = "Signup";
const userStore = userPinia();
const emits = defineEmits(["to-signin"]);

interface UserForm {
  username: string;
  name: string;
  email: string;
  password: string;
  pwdCheck: string;
}

const formSize = ref("default");
const userFormRef = ref<FormInstance>();
const userForm = reactive<UserForm>({
  username: "",
  name: "",
  email: "",
  password: "",
  pwdCheck: "",
});

const rules = reactive<FormRules<UserForm>>({
  username: [
    { required: true, message: "Please input your username", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
  name: [
    { required: true, message: "Please input your name", trigger: "blur" },
    { min: 1, max: 16, message: "Length should be 1 to 16", trigger: "blur" },
  ],
  email: [
    {
      required: true,
      message: "Please input email address",
      trigger: "blur",
    },
    {
      type: "email",
      message: "Please input correct email address",
      trigger: ["blur", "change"],
    },
  ],
  password: [
    { required: true, message: "Please input your password", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
});

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      if(userForm.password !== userForm.pwdCheck){
        ElMessage({
          type:"error",
          message:"The passwords filled in twice are different, please check!"
        })
        return;
      }else{
        const data = await api.user.signup(userForm);
        if(typeof data !== 'undefined'){
          userStore.setUser(data);
        }
      }
    } else {
      console.log("error submit!", fields);
    }
  });
};

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  formEl.resetFields();
};

const toSignin = () => {
  emits("to-signin");
};
</script>

<style lang="scss" scoped>
@import "../../../styles/views/signup.scss";
</style>
