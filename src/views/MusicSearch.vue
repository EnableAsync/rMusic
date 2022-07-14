<script setup>
import MusicInfo from "../components/MusicSearch/MusicInfo.vue";
import SearchFile from "../components/MusicSearch/SearchFile.vue";

import { ElCard } from "element-plus";
</script>

<template>
    <ElCard style="width: 1200px">
      <MusicInfo />
      <SearchFile />
    </ElCard>
</template>
