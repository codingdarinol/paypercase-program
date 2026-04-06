<script setup lang="ts">
import { ref } from "vue";
import { useAppStore } from "@/stores/app";
import AppMenuBar from "@/components/layout/AppMenuBar.vue";
import AppStatusBar from "@/components/layout/AppStatusBar.vue";
import DataViewTab from "@/components/tabs/DataViewTab.vue";
import PendingExportTab from "@/components/tabs/PendingExportTab.vue";
import DeleteDataTab from "@/components/tabs/DeleteDataTab.vue";
import SettingsDialog from "@/components/dialog/SettingsDialog.vue";
import AppToast from "@/components/layout/AppToast.vue";

const store = useAppStore();

const showSettings = ref(false);
const showStorageInfo = ref(false);

async function handleMenuEvent(event: string) {
    if (event === "open-settings") {
        showSettings.value = true;
        return;
    }

    if (event === "show-storage") {
        showStorageInfo.value = true;
        return;
    }

    if (event === "exit") {
        const { exit } = await import("@tauri-apps/plugin-process");
        await exit(0);
    }
}
</script>

<template>
    <div class="app-layout">
        <AppMenuBar
            @open-settings="handleMenuEvent('open-settings')"
            @show-storage="handleMenuEvent('show-storage')"
            @exit="handleMenuEvent('exit')"
        />

        <div class="tab-bar">
            <button
                class="tab-btn"
                :class="{ active: store.activeTab === 'data' }"
                @click="store.activeTab = 'data'"
            >
                📝 Input Data
            </button>
            <button
                class="tab-btn"
                :class="{ active: store.activeTab === 'pending' }"
                @click="store.activeTab = 'pending'"
            >
                📤 Menunggu Ekspor
            </button>
            <button
                class="tab-btn"
                :class="{ active: store.activeTab === 'delete' }"
                @click="store.activeTab = 'delete'"
            >
                🗑️ Hapus Data
            </button>
        </div>

        <div class="main-panel">
            <div class="tab-pane" v-show="store.activeTab === 'data'">
                <DataViewTab />
            </div>
            <div class="tab-pane" v-show="store.activeTab === 'pending'">
                <PendingExportTab />
            </div>
            <div class="tab-pane" v-show="store.activeTab === 'delete'">
                <DeleteDataTab />
            </div>
        </div>

        <AppStatusBar />

        <SettingsDialog v-if="showSettings" @close="showSettings = false" />

        <div
            v-if="showStorageInfo"
            class="modal-overlay"
            @click.self="showStorageInfo = false"
        >
            <div class="modal" style="max-width: 620px">
                <div class="modal__header">
                    <span class="modal__title"
                        >🗄️ Penyimpanan Data Lokal</span
                    >
                    <button
                        class="modal__close"
                        @click="showStorageInfo = false"
                    >
                        ✕
                    </button>
                </div>
                <div
                    class="modal__body"
                    style="font-size: 13px; line-height: 1.8"
                >
                    <p>
                        <strong
                            >Aplikasi ini berjalan sepenuhnya dalam mode
                            offline-first.</strong
                        >
                    </p>
                    <ul style="margin-top: 10px; padding-left: 20px">
                        <li>
                            Semua data disimpan di database SQLite lokal
                            `paypercase.db`.
                        </li>
                        <li>
                            Input data pasien dilakukan manual dari aplikasi,
                            tanpa koneksi ke server eksternal.
                        </li>
                        <li>
                            Data tetap bisa ditinjau, diekspor ke CSV, dan
                            dihapus dari perangkat ini.
                        </li>
                    </ul>
                </div>
                <div class="modal__footer">
                    <button
                        class="btn btn-primary"
                        @click="showStorageInfo = false"
                    >
                        Tutup
                    </button>
                </div>
            </div>
        </div>

        <AppToast />
    </div>
</template>

<style>
*,
*::before,
*::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html,
body,
#app {
    height: 100%;
    font-family: var(--font);
    font-size: 13px;
    color: var(--text-primary);
    background: var(--bg-main);
    overflow: hidden;
    -webkit-font-smoothing: antialiased;
    text-rendering: optimizeLegibility;
}

.app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background: var(--bg-main);
}

.tab-bar {
    display: flex;
    background: #e8eaf6;
    border-bottom: 2px solid var(--border);
    padding: 0 8px;
    flex-shrink: 0;
    gap: 2px;
}

.tab-btn {
    padding: 8px 20px;
    border: none;
    border-bottom: 3px solid transparent;
    margin-bottom: -2px;
    background: transparent;
    color: var(--text-primary);
    cursor: pointer;
    font-family: var(--font);
    font-size: 13px;
    white-space: nowrap;
    transition:
        background 0.15s,
        border-color 0.15s,
        color 0.15s;
    border-radius: var(--radius) var(--radius) 0 0;
}

.tab-btn:hover {
    background: #c5cae9;
    border-bottom-color: var(--border);
}

.tab-btn.active {
    background: #fff;
    border-bottom-color: var(--primary);
    font-weight: 600;
    color: var(--primary);
}

.main-panel {
    flex: 1;
    overflow: hidden;
    background: var(--bg-main);
}

.tab-pane {
    height: 100%;
    overflow: hidden;
}
</style>
