<template>
    <div>
        <div class="group-box mb-12">
            <span class="group-box__title">Tambah Obat atau Bahan</span>
            <div style="padding-top: 12px">
                <div
                    class="form-grid"
                    style="
                        max-width: 760px;
                        grid-template-columns: 160px minmax(0, 1fr) 160px minmax(0, 1fr);
                    "
                >
                    <label>Kode item :</label>
                    <input
                        v-model="form.icode"
                        type="text"
                        placeholder="Opsional, akan dibuat otomatis bila kosong"
                    />

                    <label>Nama singkat :</label>
                    <input
                        v-model="form.short_name"
                        type="text"
                        placeholder="Mis. Oil herbal"
                    />

                    <label>Nama lengkap :</label>
                    <input
                        v-model="form.name"
                        type="text"
                        placeholder="Nama item untuk referensi"
                    />
                </div>

                <div class="flex gap-8 mt-12">
                    <button class="btn btn-primary" @click="save">
                        Simpan Item
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
            <span class="group-box__title">Item Tersimpan</span>
            <div style="padding-top: 10px">
                <div class="table-wrapper" style="max-height: 340px">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th style="width: 40px">#</th>
                                <th style="width: 140px">Kode</th>
                                <th>Nama lengkap</th>
                                <th style="width: 180px">Nama singkat</th>
                                <th style="width: 60px">Hapus</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(item, index) in list" :key="item.id">
                                <td>{{ index + 1 }}</td>
                                <td>{{ item.icode }}</td>
                                <td>{{ item.name }}</td>
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
import type { DrugConfig } from "@/types";

type DrugForm = {
    icode: string;
    name: string;
    short_name: string;
};

const list = ref<DrugConfig[]>([]);
const form = ref<DrugForm>(createEmptyForm());
const msg = ref("");
const msgOk = ref(true);

onMounted(load);

function createEmptyForm(): DrugForm {
    return {
        icode: "",
        name: "",
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

function buildLocalCode() {
    return `DRUG-${Date.now()}`;
}

async function load() {
    try {
        list.value = await cmd.getAllDrugs();
    } catch {}
}

async function save() {
    msg.value = "";

    const icode = form.value.icode.trim() || buildLocalCode();
    const name = form.value.name.trim();
    const shortName = form.value.short_name.trim();

    if (!name) {
        msg.value = "Nama lengkap wajib diisi.";
        msgOk.value = false;
        return;
    }

    if (!shortName) {
        msg.value = "Nama singkat wajib diisi.";
        msgOk.value = false;
        return;
    }

    const duplicate = list.value.find(
        (item) =>
            item.icode.toLowerCase() === icode.toLowerCase() ||
            item.short_name.toLowerCase() === shortName.toLowerCase(),
    );

    if (duplicate) {
        msg.value = "Item dengan kode atau nama singkat yang sama sudah ada.";
        msgOk.value = false;
        return;
    }

    try {
        await cmd.saveDrug(icode, name, shortName);
        msg.value = `Item "${shortName}" berhasil disimpan.`;
        msgOk.value = true;
        resetForm(false);
        await load();
    } catch (error: any) {
        msg.value = String(error);
        msgOk.value = false;
    }
}

async function remove(id: number) {
    if (!confirm("Hapus item ini?")) return;

    try {
        await cmd.deleteDrug(id);
        await load();
        msg.value = "Item berhasil dihapus.";
        msgOk.value = true;
    } catch (error: any) {
        msg.value = String(error);
        msgOk.value = false;
    }
}
</script>
