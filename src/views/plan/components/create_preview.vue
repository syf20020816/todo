<template>
  <div id="create_preview">
    <div class="create_wrapper">
      <div class="date_wrapper">
        <span>Today is {{ date.toLocaleDateString() }}</span>
        <span>{{ week[date.getDay()] }}</span>
      </div>
      <div class="info_wrapper">
        <div class="info_item" v-for="(item, index) in infoList" :key="index">
          <div class="title">{{ item.label }}</div>
          <div class="value">{{ item.value }}</div>
        </div>
      </div>
      <div class="work_wrapper">
        <el-button type="primary">Add New TODO</el-button>
        <el-button type="danger">Remove All TODOs</el-button>
        <el-button type="success">Complete All TODOs</el-button>
      </div>
      <div class="todo_wrapper">
        <TODOItem :current-todo="currentTodo"></TODOItem>
      </div>
    </div>
    <div class="preview_wrapper">
      <div class="todo-item" v-for="item in datas" :key="item.id">
        <div class="left">
          <span class="dot_wrapper">
            <span class="priority" :style="getPriorityDot(item)"></span>
            <span
              class="priority"
              style="border-radius: 50%"
              :style="getStatusDot(item)"
            ></span>
          </span>
          <div class="todo-name">{{ item.name }}</div>
        </div>
        <div class="right">
          <el-icon size="18px" class="icons" @click="showTodoDetails(item)"
            ><QuestionFilled
          /></el-icon>
          <el-icon size="18px" class="icons"><CircleClose /></el-icon>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { TODOItem } from "../index";
import { CircleClose, QuestionFilled } from "@element-plus/icons-vue";
import {
  Todo,
  Priorities,
  Avatars,
  Status,
  usePriorityColor,
  useAvatar,
  useStatus,
} from "../../../core";
const props = defineProps<{
  datas: Todo[];
}>();
const date = new Date();

const week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const infoList = reactive([
  {
    label: "TODOs for today",
    value: 3,
  },
  {
    label: "Emergent TODOs",
    value: 1,
  },
  {
    label: "Normal TODOs",
    value: 2,
  },
  {
    label: "Start ~ End",
    value: "10.30 ~ 16.00",
  },
]);

const currentTodo = ref<any>();

const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});

const showTodoDetails = (item: Todo) => {
  currentTodo.value = item;
};
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;
#create_preview {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  overflow-y: scroll;
  scrollbar-width: thin;
  .create_wrapper {
    width: 65%;
    .date_wrapper {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: flex-start;
      span {
        margin: 0 6px;
        font-weight: 700;
      }
    }
    .info_wrapper {
      box-sizing: border-box;
      padding: 6px;
      border-top: 1px solid $bg-color-hover;
      width: 100%;
      height: fit-content;
      border-bottom: 1px solid $bg-color-hover;
      .info_item {
        font-size: 14px;
        height: 2em;
        display: flex;
        align-items: center;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0 6px;
        transition: all 0.25s ease-in-out;
        cursor: pointer;
        &:hover {
          background-color: $bg-color-hover;
          color: $force-color;
        }
      }
    }
    .work_wrapper {
      box-sizing: border-box;
      padding: 6px;
      display: flex;
      align-items: center;
      justify-content: flex-end;
    }
    .todo_wrapper {
      height: 460px;
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      box-sizing: border-box;
      padding: 12px;
    }
  }
  .preview_wrapper {
    height: 100%;
    width: 35%;
    box-sizing: border-box;
    border-left: 1px dashed $bg-color-hover;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    flex-wrap: wrap;
    align-content: flex-start;
    .todo-item {
      margin-bottom: 6px;
      height: 60px;
      width: 96%;
      background-color: $bg-color-hover;
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: flex-start;
      box-sizing: border-box;
      padding: 12px;
      .left {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        width: calc(100% - 48px);
        .dot_wrapper {
          display: inline-block;
          height: 1.5em;
          display: flex;
          align-items: center;
          justify-content: center;
          margin-right: 12px;
          .priority {
            display: inline-block;
            height: 12px;
            width: 12px;
            border-radius: 2px;
            margin: 0 4px;
          }
        }
        .todo-name {
          font-size: 18px;
          width: calc(100% - 24px);
          font-weight: 700;
          text-align: left;
        }
      }
      .right {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 48px;
        .icons {
          cursor: pointer;
        }
      }
    }
  }
}
</style>
