<template>
  <div :id="buildView(component)">
    <!-- LOGO -->
    <div v-html="useSvg(SVGs.LOGO, 24)" :class="buildWrap(component, 'logo-wrap')"></div>
    <!-- APP 名字 -->
    <span class="title">MY <span class="clip">TODO</span></span>
    <!-- 搜索框 -->
    <div class="search-wrap">
      <el-input
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
      <div
        v-html="useSvg(SVGs.SETTING, 24)"
        :class="buildWrap(component, 'logo-wrap')"
      ></div>
      <div
        v-html="useSvg(SVGs.NOTE, 24)"
        :class="buildWrap(component, 'logo-wrap')"
      ></div>
      <div :class="buildWrap(component, 'logo-wrap')">
        <img :src="userAvatar" alt="" height="48" width="48" style="border-radius: 50%" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Search } from "@element-plus/icons-vue";
import { ref, reactive, defineComponent, computed } from "vue";
import { buildView, buildWrap } from "../../core";
import { SVGs, useSvg } from "../index";
import { user } from "../../store/src/user";
const component = "Header";
defineComponent({
  name: component,
});

const userStore = user();
const searchValue = ref("");

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
</script>

<style lang="scss" scoped>
@import "../../styles/components/header.scss";
</style>
