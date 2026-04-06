<template>
    <div>
        <div class="group-box mb-12">
            <span class="group-box__title">Tambah Hak Perawatan</span>
            <div style="padding-top: 12px">
                <div
                    class="form-grid"
                    style="
                        max-width: 860px;
                        grid-template-columns: 160px minmax(0, 1fr) 160px minmax(0, 1fr);
                    "
                >
                    <label>Kode internal :</label>
                    <input
                        v-model="form.pttype"
                        type="text"
                        placeholder="Opsional, akan dibuat otomatis bila kosong"
                    />

                    <label>Nama singkat :</label>
                    <input
                        v-model="form.short_name"
                        type="text"
                        placeholder="Mis. UHC"
                    />

                    <label>Nama tampilan :</label>
                    <input
                        v-model="form.name"
                        type="text"
                        placeholder="Mis. Universal Health Coverage"
                    />

                    <label>Kode hipdata :</label>
                    <input
                        v-model="form.hipdata_code"
                        type="text"
                        placeholder="Opsional"
                    />

                    <label>Kode pcode :</label>
                    <input
                        v-model="form.pcode"
                        type="text"
                        placeholder="Opsional"
                    />
                </div>

                <p class="text-sm text-gray mt-8">
                    Kolom kode bisa dikosongkan. Aplikasi tetap dapat memakai
                    nama singkat sebagai referensi lokal.
                </p>

                <div class="flex gap-8 mt-12">
                    <button class="btn btn-primary" @click="save">
                        Simpan Hak Perawatan
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
            <span class="group-box__title">Hak Perawatan Tersimpan</span>
            <div style="padding-top: 10px">
                <div class="table-wrapper" style="max-height: 340px">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th style="width: 40px">#</th>
                                <th style="width: 140px">Kode internal</th>
                                <th>Nama tampilan</th>
                                <th style="width: 120px">Nama singkat</th>
                                <th style="width: 140px">Kode hipdata</th>
                                <th style="width: 120px">Pcode</th>
                                <th style="width: 60px">Hapus</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(item, index) in list" :key="item.id">
                                <td>{{ index + 1 }}</td>
                                <td>{{ item.pttype }}</td>
                                <td>{{ item.name }}</td>
                                <td>{{ item.short_name }}</td>
                                <td>{{ item.hipdata_code }}</td>
                                <td>{{ item.pcode }}</td>
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
                                    colspan="7"
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
import type { PttypeConfig } from "@/types";

type PttypeForm = {
    pttype: string;
    name: string;
    pcode: string;
    hipdata_code: string;
    short_name: string;
};

const list = ref<PttypeConfig[]>([]);
const form = ref<PttypeForm>(createEmptyForm());
const msg = ref("");
const msgOk = ref(true);

onMounted(load);

function createEmptyForm(): PttypeForm {
    return {
        pttype: "",
        name: "",
        pcode: "",
        hipdata_code: "",
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

function normalizeToken(value: string) {
    return value.trim().replace(/\s+/g, "_").toUpperCase();
}

async function load() {
    try {
        list.value = await cmd.getAllPttypes();
    } catch {}
}

async function save() {
    msg.value = "";

    const name = form.value.name.trim();
    const shortName = form.value.short_name.trim();
    const pttype = form.value.pttype.trim() || normalizeToken(shortName);
    const hipdataCode =
        form.value.hipdata_code.trim() || normalizeToken(shortName);
    const pcode = form.value.pcode.trim();

    if (!name) {
        msg.value = "Nama tampilan wajib diisi.";
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
            item.short_name.toLowerCase() === shortName.toLowerCase() ||
            item.pttype.toLowerCase() === pttype.toLowerCase() ||
            item.hipdata_code.toLowerCase() === hipdataCode.toLowerCase(),
    );

    if (duplicate) {
        msg.value =
            "Hak perawatan dengan nama singkat atau kode yang sama sudah ada.";
        msgOk.value = false;
        return;
    }

    try {
        await cmd.savePttype(pttype, name, pcode, hipdataCode, shortName);
        msg.value = `Hak perawatan "${shortName}" berhasil disimpan.`;
        msgOk.value = true;
        resetForm(false);
        await load();
    } catch (error: any) {
        msg.value = String(error);
        msgOk.value = false;
    }
}

async function remove(id: number) {
    if (!confirm("Hapus hak perawatan ini?")) return;

    try {
        await cmd.deletePttype(id);
        await load();
        msg.value = "Hak perawatan berhasil dihapus.";
        msgOk.value = true;
    } catch (error: any) {
        msg.value = String(error);
        msgOk.value = false;
    }
}
</script>
