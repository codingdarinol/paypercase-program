import { defineStore } from "pinia";
import { computed, ref } from "vue";

export type MainTab = "data" | "pending" | "delete";

export const useAppStore = defineStore("app", () => {
    const activeTab = ref<MainTab>("data");

    function setTab(tab: MainTab) {
        activeTab.value = tab;
    }

    const pendingRefreshKey = ref(0);

    function triggerPendingRefresh() {
        pendingRefreshKey.value++;
    }

    const statusText = computed(
        () => "Mode offline aktif • data disimpan di SQLite lokal",
    );

    return {
        activeTab,
        setTab,
        pendingRefreshKey,
        triggerPendingRefresh,
        statusText,
    };
});
