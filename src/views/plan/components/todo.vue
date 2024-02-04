<template>
  <div class="todo" v-if="typeof currentTodo !== 'undefined'">
    <div class="header">
      <span class="dot_wrapper">
        <span class="priority" :style="getPriorityDot(currentTodo)"></span>
      </span>
      <div class="todo-name">{{ currentTodo.name }}</div>
    </div>
    <div class="details">
      <div class="left">
        <div class="tags">
          tag :
          <el-tag
            v-for="tag in currentTodo.tags"
            :key="tag.label"
            :type="tag.type"
            size="small"
            class="mx-tag"
            :effect="tag.effect"
          >
            {{ tag.label }}
          </el-tag>
        </div>
        <div class="reviews">
          <div style="text-align: left">Reviews:</div>
          <div
            class="review-item"
            v-for="review in currentTodo.reviewers"
            :key="review.id"
          >
            <img class="review-avatar" :src="useAvatar(review.avatar)" alt="" />
            <div class="review-info">
              <div class="name">{{ review.name }}</div>
              <div class="email">{{ review.email }}</div>
            </div>
          </div>
        </div>
        <div class="reviews">
          <div style="text-align: left">Performers:</div>
          <div
            class="review-item"
            v-for="review in currentTodo.performers"
            :key="review.id"
          >
            <img class="review-avatar" :src="useAvatar(review.avatar)" alt="" />
            <div class="review-info">
              <div class="name">{{ review.name }}</div>
              <div class="email">{{ review.email }}</div>
            </div>
          </div>
        </div>
      </div>
      <div class="right">
        <div class="date">
          <div style="text-align: left">
            DateTime:{{ currentTodo.date.start }} ~ {{ currentTodo.date.end }}
          </div>
          <div style="text-align: right">During : {{ currentTodo.date.during }}</div>
        </div>
        <div class="status">
          <span style="margin-right: 6px">Status :</span>
          <el-tag
            round
            :color="useStatus(currentTodo.status)"
            class="mx-tag"
            effect="dark"
          >
            {{ currentTodo.status }}
          </el-tag>
        </div>
        <div class="des">
          <span style="font-weight: 700">Description :</span>
          <p>{{ currentTodo.description }}</p>
        </div>
        <div class="more">
          <span>Annex and Details :</span>
          <el-button type="default">Download</el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import {
  ITODO,
  PriorityEnum,
  usePriorityColor,
  useAvatar,
  useStatus,
} from "../../../core";

const props = defineProps<{
  currentTodo?: ITODO;
}>();

const getPriorityDot = computed(() => (item: ITODO) => {
  let { priority } = item || PriorityEnum.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;
.todo {
  color: $bg-color-light;
  height: 100%;
  border-radius: 4px;
  background-color: $bg-color-hover;
  width: 100%;
  box-sizing: border-box;
  padding: 16px;
  .header {
    height: 60px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .dot_wrapper {
      display: inline-block;
      height: 1.5em;
      display: flex;
      align-items: center;
      justify-content: center;
      .priority {
        display: inline-block;
        height: 12px;
        width: 12px;
        border-radius: 2px;
        background-color: #d02828;
      }
    }
    .todo-name {
      font-size: 18px;
      width: calc(100% - 24px);
      font-weight: 700;
      text-align: left;
    }
  }
  .details {
    border-top: 1px solid $bg-color-light;
    padding-top: 12px;
    width: 100%;
    height: calc(100% - 72px);
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    .left {
      height: 100%;
      width: 36%;
      .tags {
        height: 24px;
        width: 100%;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: flex-start;
      }
      .mx-tag {
        margin: 0 4px;
      }
      .reviews {
        margin: 6px 0;
        height: calc(50% - 12px);
        overflow-y: scroll;
        scrollbar-width: thin;
        .review-item {
          padding: 0 6px;
          box-sizing: border-box;
          cursor: pointer;
          height: 60px;
          width: 96%;
          display: flex;
          align-items: center;
          justify-content: space-between;
          transition: all 0.25s ease-in-out;
          background-color: $bg-color-menu;
          &:hover {
            background-color: $bg-color-header;
          }
          .review-avatar {
            height: 32px;
            width: 32px;
            border-radius: 50%;
          }
          .review-info {
            box-sizing: border-box;
            padding: 0 12px;
            width: calc(100% - 36px);
            .name {
              text-align: left;
              font-size: 14px;
              font-weight: 700;
              color: $force-color;
            }
            .email {
              overflow: hidden;
              text-align: left;
              font-size: 12px;
            }
          }
        }
      }
    }
    .right {
      height: 100%;
      width: 64%;
      .date {
        box-sizing: border-box;
        padding-bottom: 6px;
        border-bottom: 1px solid $bg-color-light;
      }
      .status {
        padding-top: 6px;
        height: 24px;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: flex-start;
      }
      .des {
        box-sizing: border-box;
        padding-top: 6px;
        overflow: hidden;
        height: calc(100% - 120px);
        text-align: left;
        p {
          margin: 6px;
        }
      }
      .more {
        display: flex;
        align-items: center;
        justify-content: space-between;
      }
    }
  }
}
</style>
