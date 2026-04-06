<template>
    <div>
        <div class="group-box mb-12">
            <span class="group-box__title">Tambah Terapis</span>
            <div style="padding-top: 12px">
                <div
                    class="form-grid"
                    style="
                        max-width: 760px;
                        grid-template-columns: 160px minmax(0, 1fr) 160px minmax(0, 1fr);
                    "
                >
                    <label>ID lokal :</label>
                    <input
                        v-model="form.health_med_provider_id"
                        type="number"
                        min="1"
                        placeholder="Opsional, akan dibuat otomatis bila kosong"
                    />

                    <label>Nama singkat :</label>
                    <input
                        v-model="form.short_name"
                        type="text"
                        placeholder="Mis. Terapis A"
                    />

                    <label>Nama lengkap :</label>
                    <input
                        v-model="form.full_name"
                        type="text"
                        placeholder="Nama lengkap terapis"
                    />
                </div>

                <p class="text-sm text-gray mt-8">
                    ID lokal hanya dipakai sebagai penanda internal di SQLite.
                </p>

                <div class="flex gap-8 mt-12">
                    <button class="btn btn-primary" @click="save">
                        Simpan Terapis
                    </button>
                    <button class="btn btn-ghost" @click="clearForm">
                        Reset
                    </button>
                </div>

                <div
                    v-if="msg"
                    class="alert mt-8"
                    :class="msgOk ? 'alert-success' : 'alert-error'"
                >
                    {{ msg }}
                </div>
            </div>
        </div>

        <div class="group-box">
            <span class="group-box__title">Terapis Tersimpan</span>
            <div style="padding-top: 10px">
                <div class="table-wrapper" style="max-height: 340px">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th style="width: 40px">#</th>
                                <th style="width: 140px">ID lokal</th>
                                <th>Nama lengkap</th>
                                <th style="width: 180px">Nama singkat</th>
                                <th style="width: 60px">Hapus</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(item, index) in list" :key="item.id">
                                <td>{{ index + 1 }}</td>
                                <td>{{ item.health_med_provider_id }}</td>
                                <td>{{ item.full_name }}</td>
                                <td>{{ item.short_name }}</td>
                                <td>
                                    <button
                                        class="btn btn-danger btn-sm"
                                        @click="remove(item.id)"
                                    >
                                        Hapus
                                    </button>
                                </td>
                            </tr>
                            <tr v-if="list.length === 0">
                                <td
                                    colspan="5"
                                    style="
                                        text-align: center;
                                        color: var(--text-gray);
                                        padding: 20px;
                                    "
                                >
                                    Belum ada data.
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import * as cmd from "@/composables/useCommands";
import type { ProviderConfig } from "@/types";

type ProviderForm = {
    health_med_provider_id: string;
    full_name: string;
    short_name: string;
};

const list = ref<ProviderConfig[]>([]);
const form = ref<ProviderForm>(createEmptyForm());
const msg = ref("");
const msgOk = ref(true);

onMounted(load);

function createEmptyForm(): ProviderForm {
    return {
        health_med_provider_id: "",
        full_name: "",
        short_name: "",
    };
}

function resetForm(clearMessage = true) {
    form.value = createEmptyForm();
    if (clearMessage) {
        msg.value = "";
    }
}

function clearForm() {
    resetForm(true);
}

function buildLocalId() {
    return Date.now().toString();
}

async function load() {
    try {
        list.value = await cmd.getAllProviders();
    } catch {}
}

async function save() {
    msg.value = "";

    const fullName = form.value.full_name.trim();
    const shortName = form.value.short_name.trim();
    const rawId = form.value.health_med_provider_id.trim() || buildLocalId();
    const providerId = Number(rawId);

    if (!fullName) {
        msg.value = "Nama lengkap wajib diisi.";
        msgOk.value = false;
        return;
    }

    if (!shortName) {
        msg.value = "Nama singkat wajib diisi.";
        msgOk.value = false;
        return;
    }

    if (!Number.isInteger(providerId) || providerId <= 0) {
        msg.value = "ID lokal harus berupa angka bulat positif.";
        msgOk.value = false;
        return;
    }

    const duplicate = list.value.find(
        (item) =>
            item.health_med_provider_id === providerId ||
            item.full_name.toLowerCase() === fullName.toLowerCase() ||
            item.short_name.toLowerCase() === shortName.toLowerCase(),
    );

    if (duplicate) {
        msg.value = "Terapis dengan ID atau nama yang sama sudah ada.";
        msgOk.value = false;
        return;
    }

    try {
        await cmd.saveProvider(providerId, fullName, shortName);
        msg.value = `Terapis "${shortName}" berhasil disimpan.`;
        msgOk.value = true;
        resetForm(false);
        await load();
    } catch (error: any) {
        msg.value = String(error);
        msgOk.value = false;
    }
}

async function remove(id: number) {
    if (!confirm("Hapus terapis ini?")) return;

    try {
        await cmd.deleteProvider(id);
        await load();
        msg.value = "Terapis berhasil dihapus.";
        msgOk.value = true;
    } catch (error: any) {
        msg.value = String(error);
        msgOk.value = false;
    }
}
</script>
