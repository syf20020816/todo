<template>
  <div :id="buildView(component)">
    <!-- LOGO -->
    <div v-html="useSvg(SVGs.LOGO, 24)" :class="buildWrap(component, 'logo-wrap')"></div>
    <!-- APP 名字 -->
    <span class="title">MY <span class="clip">TODO</span></span>
    <!-- 搜索框 -->
    <div class="search-wrap">
      <el-input
        v-if="userStore.isSignIn"
        v-model="searchValue"
        placeholder="Search TODOs"
        class="input-with-select"
      >
        <template #prepend>
          <div>TODO</div>
        </template>
        <template #append>
          <el-button :icon="Search" @click="onSearch" />
        </template>
      </el-input>
    </div>
    <!-- 右侧工具栏 -->
    <div :class="buildWrap(component, 'right-wrap')">
      <div v-if="userStore.isSignIn">
        <div
          @click="openSetting"
          v-html="useSvg(SVGs.SETTING, 24)"
          :class="buildWrap(component, 'logo-wrap')"
        ></div>
        <div
          v-html="useSvg(SVGs.NOTE, 24)"
          :class="buildWrap(component, 'logo-wrap')"
        ></div>
        <div :class="buildWrap(component, 'logo-wrap')">
          <img
            :src="userAvatar"
            alt=""
            height="48"
            width="48"
            style="border-radius: 50%"
          />
        </div>
      </div>
    </div>
    <el-drawer
      ref="settingDrawerRef"
      v-model="settingDrawer"
      :before-close="handleClose"
      direction="rtl"
      class="setting-drawer"
      size="460px"
    >
      <template #header>
        <div class="setting-drawer-title">Personal Settings</div>
      </template>
      <div class="setting-drawer-content">
        <el-form
          label-position="left"
          ref="ruleFormRef"
          :model="ruleForm"
          :rules="rules"
          label-width="120px"
          class="demo-ruleForm"
          :size="formSize"
          status-icon
        >
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                The name is arbitrary, but it is recommended to use a real name
              </p>
            </el-alert>
            <el-form-item label="Name" prop="name">
              <el-input v-model="ruleForm.name" />
            </el-form-item>
          </el-space>
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                Email is a required field. Please ensure that the email you filled in is
                your work email and can be received normally
              </p>
            </el-alert>
            <el-form-item label="Email" prop="email">
              <el-input v-model="ruleForm.email" />
            </el-form-item>
          </el-space>
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                Enabling means that when you complete TODO, the system will send an email
                to the reviewer
              </p>
            </el-alert>
            <el-form-item label="Send Email" prop="sendEmail">
              <el-switch v-model="ruleForm.sendEmail" />
            </el-form-item>
          </el-space>
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                Enabling means that when you complete TODO, the system will send a system
                message to the reviewer
              </p>
            </el-alert>
            <el-form-item label="Send Message" prop="sendMessage">
              <el-switch v-model="ruleForm.sendMessage" />
            </el-form-item>
          </el-space>
          <el-form-item>
            <el-button type="primary" @click="submitForm(ruleFormRef)">
              Save Settings
            </el-button>
            <el-button @click="resetForm(ruleFormRef)">Reset</el-button>
          </el-form-item>
        </el-form>
      </div>
    </el-drawer>
  </div>
</template>

<script lang="ts" setup>
import { Search } from "@element-plus/icons-vue";
import { ref, reactive, defineComponent, computed } from "vue";
import { buildView, buildWrap } from "../../core";
import { SVGs, useSvg } from "../index";
import { user } from "../../store/src/user";
import type { FormInstance, FormRules } from "element-plus";

const component = "Header";
defineComponent({
  name: component,
});

const userStore = user();
const searchValue = ref("");
const settingDrawer = ref(false);

/**
 * 计算得出用户头像
 */
const userAvatar = computed(() => {
  return userStore.useAvatar(userStore.userInfo.avatar);
});

/**
 * 查询待办项事件
 */
const onSearch = () => {
  //去除字符串前后空格
  const formatSearchValue = searchValue.value.trim();
  console.log(formatSearchValue);
};

const openSetting = () => {
  settingDrawer.value = true;
};

const handleClose = () => {
  settingDrawer.value = false;
};

interface RuleForm {
  name: string;
  email: string;
  sendEmail: boolean;
  sendMessage: boolean;
}

const formSize = ref("default");
const ruleFormRef = ref<FormInstance>();
const ruleForm = reactive<RuleForm>({
  name: "Hello",
  email: "",
  sendEmail: false,
  sendMessage: false,
});

const rules = reactive<FormRules<RuleForm>>({
  name: [
    { required: true, message: "Please input Activity name", trigger: "blur" },
    { min: 3, max: 5, message: "Length should be 3 to 5", trigger: "blur" },
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
});

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log("submit!");
    } else {
      console.log("error submit!", fields);
    }
  });
};

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  formEl.resetFields();
};

const options = Array.from({ length: 10000 }).map((_, idx) => ({
  value: `${idx + 1}`,
  label: `${idx + 1}`,
}));
</script>

<style lang="scss" scoped>
@import "../../styles/components/header.scss";
</style>
