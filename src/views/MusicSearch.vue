<script setup>
import MusicInfo from "../components/MusicSearch/MusicInfo.vue";
import SearchFile from "../components/MusicSearch/SearchFile.vue";

import { ElCard } from "element-plus";
</script>

<template>
  <main>
    <ElCard style="width: 800px">
      <MusicInfo />
      <SearchFile />
    </ElCard>
  </main>
</template>
