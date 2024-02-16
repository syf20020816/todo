<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'left')">
      <el-calendar>
        <template #date-cell="{ data }">
          <div @click="getTODO(data)" style="height: 100%; width: 100%">
            <p :class="data.isSelected ? 'is-selected' : ''">
              {{ data.day.split("-").slice(1).join("-") }}
              {{ data.isSelected ? "🚩" : "" }}
            </p>
          </div>
        </template>
      </el-calendar>
    </div>
    <div :class="buildWrap(component, 'mid')">
      <el-timeline v-if="todos?.length">
        <el-timeline-item
          v-for="(item, index) in todos"
          :key="index"
          :timestamp="`${item.date.start}~${item.date.end}`"
        >
          <h4>{{ item.name }}</h4>
          <el-button type="primary" @click="currentItem = item">📃Check</el-button>
        </el-timeline-item>
      </el-timeline>
      <div v-else>
        <h4>There are no history TODOs for the current date</h4>
      </div>
    </div>
    <div :class="buildWrap(component, 'right')">
      <TODOItem :current-todo="currentItem" :is-change="false"></TODOItem>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
import {
  AvatarMap,
  Avatars,
  Todo,
  Priorities,
  Status,
  TeamAvatars,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../core";
import { TODOItem } from "./plan";
import { user } from "../store/src/user";

const component = "History";
const userStore = user();

const currentItem = ref<Todo>();

const todos = ref<Todo[]>();
const filterTodos = (date: Date) => {
  let time = new Date(date.toLocaleDateString()).getTime();
  todos.value = userStore.user.todos.history.filter((todo) => {
    let end = new Date(new Date(todo.date.end).toLocaleDateString()).getTime();
    let start = new Date(new Date(todo.date.start).toLocaleDateString()).getTime();

    if (start <= time && end >= time) {
      return true;
    }
    return false;
  });
};

const getTODO = (data: any) => {
  let date = data.date;
  currentItem.value = undefined;
  filterTodos(date);
};

filterTodos(new Date());
</script>

<style lang="scss" scoped>
@import "../styles/views/history.scss";
</style>
