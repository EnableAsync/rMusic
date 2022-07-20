import { createRouter, createWebHistory } from "vue-router";
import BaseLayout from "../layout/BaseLayout.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      redirect: "/search",
      component: BaseLayout,
      children: [
        {
          path: "/user",
          name: "user",
          component: () => import("../views/UserView.vue"),
        },
        {
          path: "/drive",
          name: "drive",
          component: () => import("../views/DriveView.vue"),
        },
        {
          path: "/search",
          name: "search",
          component: () => import("../views/MusicSearch.vue"),
        },
      ],
    },
  ],
});

export default router;
