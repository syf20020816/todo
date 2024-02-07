<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'teams')">
      <div v-for="item in teamList" :key="item.id" class="team_wrapper">
        <div class="team-avatar">
          <img :src="useTeam(item.teamIcon)" alt="" class="teamIcons" />
        </div>
        <div class="team-details">
          <h5 class="name">{{ item.name }}</h5>
          <p class="desc">{{ item.description }}</p>
        </div>
      </div>
    </div>
    <div :class="buildWrap(component, 'details')"></div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
import {
  AvatarMap,
  Avatars,
  ITODO,
  PriorityEnum,
  Status,
  Teams,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../core";
import { Timeline } from "./plan";
const component = "Collaborate";

const teamList = reactive<
  {
    id: string;
    name: string;
    role: string;
    teamIcon: Teams;
    description: string;
  }[]
>([
  {
    id: "0",
    name: "Surrealism",
    role: "Manager",
    teamIcon: Teams.Team1,
    description: "Surrealism is a SQL Builder",
  },
  {
    id: "1",
    name: "SurrealismUI",
    role: "Manager",
    teamIcon: Teams.Team2,
    description: "Surrealism is a SQL Builder",
  },
  {
    id: "2",
    name: "Slimk",
    role: "Watcher",
    teamIcon: Teams.Team3,
    description: "Surrealism is a SQL Builder",
  },
]);
</script>

<style lang="scss" scoped>
@import "../styles/views/collaborate.scss";
</style>
