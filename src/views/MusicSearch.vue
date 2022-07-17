<script setup>
import SearchFile from "../components/MusicSearch/SearchFile.vue";
import AudioPlayer from "vue3-audio-player";
import { ElCard, ElRow, ElCol } from "element-plus";
import { usePlayerStore } from "../stores/player";

const player = usePlayerStore();
</script>

<template>
  <ElRow>
    <ElCol :span="24">
      <ElCard>
        <SearchFile />
      </ElCard>
    </ElCol>
  </ElRow>

  <ElRow>
    <ElCol :span="24">
      <AudioPlayer
        :option="player.song"
        style="position: fixed; bottom: 0; width: 100%"
      />
    </ElCol>
  </ElRow>
</template>
