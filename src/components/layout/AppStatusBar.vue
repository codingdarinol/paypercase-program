<template>
    <div class="status-bar">
        <span class="text-sm" style="color: #9fa8da">
            Mode lokal aktif
        </span>
        <span class="flex-1" />
        <span class="status-bar__connected">🟢 {{ statusText }}</span>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useAppStore } from "@/stores/app";

const store = useAppStore();
const statusText = computed(() => store.statusText);
</script>
