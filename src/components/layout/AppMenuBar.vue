<template>
    <nav class="menu-bar">
        <span class="menu-bar__logo">
            <svg
                width="20"
                height="20"
                viewBox="0 0 64 64"
                xmlns="http://www.w3.org/2000/svg"
                style="
                    display: inline-block;
                    vertical-align: middle;
                    margin-right: 6px;
                "
            >
                <defs>
                    <linearGradient
                        id="mbMainGradient"
                        x1="0%"
                        y1="0%"
                        x2="100%"
                        y2="100%"
                    >
                        <stop
                            offset="0%"
                            style="stop-color: #7986cb; stop-opacity: 1"
                        />
                        <stop
                            offset="100%"
                            style="stop-color: #3f51b5; stop-opacity: 1"
                        />
                    </linearGradient>
                    <linearGradient
                        id="mbCircleFaceGradient"
                        x1="0%"
                        y1="0%"
                        x2="100%"
                        y2="100%"
                    >
                        <stop
                            offset="0%"
                            style="stop-color: #ffe0b2; stop-opacity: 1"
                        />
                        <stop
                            offset="100%"
                            style="stop-color: #ffab91; stop-opacity: 1"
                        />
                    </linearGradient>
                </defs>
                <line
                    x1="48"
                    y1="16"
                    x2="16"
                    y2="48"
                    stroke="url(#mbMainGradient)"
                    stroke-width="7"
                    stroke-linecap="round"
                />
                <circle
                    cx="20"
                    cy="20"
                    r="10"
                    fill="url(#mbCircleFaceGradient)"
                />
                <circle cx="20" cy="20" r="3" fill="white" fill-opacity="0.4" />
                <circle
                    cx="44"
                    cy="44"
                    r="10"
                    fill="url(#mbCircleFaceGradient)"
                />
                <circle
                    cx="44"
                    cy="44"
                    r="3"
                    fill="white"
                    fill-opacity="0.4"
                />
            </svg>
            PayPerCase
        </span>

        <div
            class="menu-bar__item"
            :class="{ active: openMenu === 'settings' }"
            @click="toggleMenu('settings')"
        >
            Pengaturan
            <div v-if="openMenu === 'settings'" class="dropdown-menu">
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('open-settings')"
                >
                    Master Data
                </div>
            </div>
        </div>

        <div
            class="menu-bar__item"
            :class="{ active: openMenu === 'about' }"
            @click="toggleMenu('about')"
        >
            Tentang
            <div v-if="openMenu === 'about'" class="dropdown-menu">
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('show-storage')"
                >
                    Penyimpanan Lokal
                </div>
                <div class="dropdown-menu__item" @click.stop="openDisclaimer">
                    Penafian
                </div>
                <div class="dropdown-menu__separator" />
                <div class="dropdown-menu__item" @click.stop="closeAndEmit('exit')">
                    Keluar
                </div>
            </div>
        </div>
    </nav>

    <div
        v-if="showDisclaimer"
        class="modal-backdrop"
        @click.self="closeDisclaimer"
    >
        <div
            class="modal"
            role="dialog"
            aria-modal="true"
            aria-label="Penafian"
        >
            <header class="modal__header">Penafian</header>
            <div class="modal__body">
                Pengembang tidak bertanggung jawab atas kerusakan atau
                kehilangan data akibat penggunaan perangkat lunak ini tanpa
                prosedur backup yang memadai.
            </div>
            <footer class="modal__footer">
                <button class="btn" @click="closeDisclaimer">Tutup</button>
            </footer>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

type MenuEmitEvent = "open-settings" | "show-storage" | "exit";

const emit = defineEmits<{
    "open-settings": [];
    "show-storage": [];
    exit: [];
}>();

const openMenu = ref<string | null>(null);
const showDisclaimer = ref(false);

function toggleMenu(name: string) {
    openMenu.value = openMenu.value === name ? null : name;
}

function closeMenu() {
    openMenu.value = null;
}

function closeAndEmit(event: MenuEmitEvent) {
    closeMenu();
    switch (event) {
        case "open-settings":
            emit("open-settings");
            break;
        case "show-storage":
            emit("show-storage");
            break;
        case "exit":
            emit("exit");
            break;
    }
}

function openDisclaimer() {
    closeMenu();
    showDisclaimer.value = true;
}

function closeDisclaimer() {
    showDisclaimer.value = false;
}

function onClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest(".menu-bar__item")) {
        closeMenu();
    }
}

onMounted(() => document.addEventListener("click", onClickOutside));
onUnmounted(() => document.removeEventListener("click", onClickOutside));
</script>

<style scoped>
.menu-bar__logo {
    color: #c5cae9;
    font-weight: bold;
    font-size: 13px;
    padding: 4px 12px;
    user-select: none;
}

.modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal {
    background: var(--bg-surface, #fff);
    padding: 16px;
    border-radius: 8px;
    width: 420px;
    max-width: calc(100% - 40px);
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.2);
}

.modal__header {
    font-weight: 600;
    margin-bottom: 8px;
}

.modal__body {
    font-size: 14px;
    color: var(--text, #333);
    margin-bottom: 12px;
}

.modal__footer {
    text-align: right;
}

.btn {
    background: var(--primary, #3f51b5);
    color: #fff;
    border: none;
    padding: 6px 10px;
    border-radius: 4px;
    cursor: pointer;
}
</style>
