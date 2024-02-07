<template>
  <div id="table-view">
    <el-table :data="datas">
      <el-table-column type="index" label="Index" width="100px" fixed="left" />
      <el-table-column label="Name" prop="name" width="200px" />
      <el-table-column label="Priority" prop="priority" width="200px" />
      <el-table-column label="Reviewers" width="200px" prop="reviewers">
        <template #default="{ row, $index }">
          <div>{{ getReviewerName(row, $index) }}</div>
        </template>
      </el-table-column>
      <el-table-column label="Performers" prop="performers" width="200px">
        <template #default="{ row, $index }">
          <div>{{ getPerformersName(row, $index) }}</div>
        </template>
      </el-table-column>
      <el-table-column label="Date" prop="date" width="200px">
        <template #default="{ row, $index }">
          <div>{{ getDate(row, $index) }}</div>
        </template>
      </el-table-column>
      <el-table-column label="Tags" prop="tags" width="200px">
        <template #default="{ row, $index }">
          <span>
            <el-tag
              style="margin: 0 4px"
              v-for="tag in getTags(row, $index)"
              :key="tag.label"
              :type="tag.type"
              size="small"
              class="mx-tag"
              :effect="tag.effect"
              >{{ tag.label }}</el-tag
            >
          </span>
        </template>
      </el-table-column>
      <el-table-column align="right" width="200px" fixed="right">
        <template #header>
          <el-input v-model="search" size="small" placeholder="Type to search" />
        </template>
        <template #default="scope">
          <el-button size="small" @click="handleEdit(scope.$index, scope.row)"
            >Edit</el-button
          >
          <el-button
            size="small"
            type="danger"
            @click="handleDelete(scope.$index, scope.row)"
            >Delete</el-button
          >
        </template>
      </el-table-column>
      <el-table-column type="expand" fixed="right">
        <template #default="props">
          <div class="expand-table-wrapper">
            <TODOItem :current-todo="props.row"></TODOItem>
          </div>
        </template>
      </el-table-column>
    </el-table>
    <div class="table-pagination-wrapper">
      <el-pagination
        background
        :page-size="20"
        :pager-count="11"
        layout="prev, pager, next"
        :total="1000"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { ITODO } from "../../../core";
import { TODOItem } from "../index";
const props = defineProps<{
  datas: ITODO[];
}>();

const search = ref("");

const handleEdit = (index: number, row: any) => {
  console.log(index, row);
};
const handleDelete = (index: number, row: any) => {
  console.log(index, row);
};

const getReviewerName = computed(() => (row: any, index: number) => {
  return row.reviewers[index]?.name ?? "";
});

const getPerformersName = computed(() => (row: any, index: number) => {
  return row.performers[index]?.name ?? "";
});

const getDate = computed(() => (row: any, _index: number) => {
  let { start, end } = row.date ?? {
    start: "",
    end: "",
  };
  return `${start ?? ""} ~ ${end ?? ""}`;
});

const getTags = computed(() => (row: any, _index: number): {
  type: string;
  effect: string;
  label: string;
}[] => {
  return row.tags;
});
</script>

<style lang="scss">
@use "../../../styles/src/var.scss" as *;
#table-view {
  box-sizing: border-box;
  height: 100%;
  // padding: 6px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  overflow-y: scroll;
  scrollbar-width: thin;
  flex-wrap: wrap;
  // 覆盖el-table默认颜色
  // 注意：这里覆盖了element-plus的样式，如果需要使用element-plus的样式，需要将element-plus的样式覆盖
  // 例如：@import "element-plus/theme-chalk/src/common/var.scss";
  // 或者使用!important来覆盖样式
  .el-table {
    --el-table-bg-color: #3b3a39;
    --el-table-header-bg-color: #252423;
    --el-table-tr-bg-color: #3b3a39;
    --el-table-row-hover-bg-color: #252423;
    --el-table-border: 1px solid #3b3a39;
    --el-table-border-color: #3b3a39;
    --el-table-text-color: #fff;
    --el-table-header-text-color: #bfbfbf;
    --el-table-expanded-cell-bg-color: #252423;
    .el-table__body-wrapper tr td.el-table-fixed-column--left {
      background: #3b3a39;
    }
    .el-table__body-wrapper tr td.el-table-fixed-column--right {
      background: #3b3a39;
    }
  }
  .expand-table-wrapper {
    width: calc(100% - 120px);
    display: flex;
    align-items: center;
    box-sizing: border-box;
    padding: 16px;
    justify-content: center;
  }
  .table-pagination-wrapper {
    position: fixed;
    bottom: 6px;
    right: 6px;
    z-index: 11;
    background-color: #11100f;
  }
}
</style>
