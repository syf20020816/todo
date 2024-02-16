<template>
  <div id="calendar-view">
    <div class="left">
      <el-calendar>
        <template #date-cell="{ data }">
          <!-- 解决点击触发范围问题 -->
          <div @click="getTODO(data)" style="height: 100%; width: 100%">
            <p :class="data.isSelected ? 'is-selected' : ''">
              {{ data.day.split("-").slice(1).join("-") }}
              {{ data.isSelected ? "🚩" : "" }}
            </p>
          </div>
        </template>
      </el-calendar>
    </div>
    <div class="right">
      <el-collapse accordion v-if="todos?.length">
        <el-collapse-item :name="item.id" v-for="item in todos" :key="item.id">
          <template #title>
            <div class="collapse-title-wrapper">
              <div>
                <span class="priority" :style="getPriorityDot(item)"></span>
                <span
                  class="priority"
                  style="border-radius: 50%"
                  :style="getStatusDot(item)"
                ></span>
                <span class="collapse-title">{{ item.name }}</span>
              </div>
            </div>
          </template>
          <div style="height: 360px">
            <TODOItem
              :current-todo="item"
              :is-change="false"
              @refresh="refreshTodo"
              @delete="refreshTodo"
            ></TODOItem>
          </div>
        </el-collapse-item>
      </el-collapse>
      <div v-else>
        <h3>Wishing you a pleasant day</h3>
        <p>There are no TODOs to be processed for the current date</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed, onMounted } from "vue";
import { Todo, Priorities, usePriorityColor, Status, useStatus } from "../../../core";
import { TODOItem } from "../index";

const props = defineProps<{ datas: Todo[] }>();
const emits = defineEmits(["getDate"]);

const todos = ref<Todo[]>();
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item;
  return `background-color : ${usePriorityColor(priority || Priorities.Low)}`;
});

const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});

const filterTodos = (date: Date) => {
  let time = new Date(date.toLocaleDateString()).getTime();
  todos.value = props.datas.filter((todo) => {
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
  console.log(date);
  filterTodos(date);
};

const refreshTodo = (_id: string) => {
  console.log(_id);
};
filterTodos(new Date());
</script>

<style lang="scss">
@use '../../../styles/src/var.scss' as *;
#calendar-view {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  .left {
    height: 100%;
    box-sizing: border-box;
    padding: 0 6px;
    width: 50%;
    .el-calendar {
      background-color: $bg-color-hover;
    }
    .el-calendar-day {
      &:hover {
        background-color: $force-color !important;
      }
    }
    .current {
      &:hover {
        background-color: $force-color !important;
      }
    }
    .is-today {
      color: $bg-color-dark !important;
    }
    :deep(.el-calendar-table td.is-selected) {
      background-color: $force-color !important;
    }
    .is-selected {
      // background-color: var(--el-calendar-selected-bg-color);
      background-color: $force-color;
    }
  }
  .right {
    height: 100%;
    width: 50%;
    padding: 0 6px;
    box-sizing: border-box;
    border-left: 1px dashed $bg-color-hover;
    overflow-y: scroll;
    scrollbar-width: thin;
    .priority {
      display: inline-block;
      height: 12px;
      width: 12px;
      border-radius: 2px;
      margin: 0 4px;
    }
    .collapse-title-wrapper {
      box-sizing: border-box;
      padding: 0 8px;
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .collapse-title {
        font-weight: 700;
        color: $bg-color-light;
      }
    }

    .el-collapse-item__header {
      background-color: $bg-color-menu !important;
    }
    .el-collapse-item__content {
      margin: 0;
      padding: 0;
      background-color: $bg-color-hover !important;
    }
  }
}
</style>
