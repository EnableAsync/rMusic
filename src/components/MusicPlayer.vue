<script setup>
import AudioPlayer from "vue3-audio-player";
import { usePlayerStore } from "../stores/player";

const player = usePlayerStore();
</script>

<template>
  <AudioPlayer
    :option="player.song"
    style="position: fixed; bottom: 0; width: 100%"
  />
</template>
