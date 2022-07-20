<script setup>
import SearchFile from "../components/MusicSearch/SearchFile.vue";
import { ElCard, ElRow, ElCol } from "element-plus";
</script>

<template>
  <ElRow>
    <ElCol :span="24">
      <ElCard>
        <SearchFile />
      </ElCard>
    </ElCol>
  </ElRow>
</template>
