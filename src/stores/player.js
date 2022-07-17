import { defineStore } from "pinia";

export const usePlayerStore = defineStore({
  id: "player",
  state: () => ({
    song: {},
  }),
  actions: {
    play(song) {
      this.song = song;
    },
  },
});
