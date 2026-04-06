<template>
    <div class="flex-col h-full" style="gap: 8px; padding: 12px 12px 8px">
        <div class="group-box" style="flex-shrink: 0">
            <span class="group-box__title">Input Data Offline</span>
            <div
                class="flex items-center gap-12"
                style="padding-top: 8px; flex-wrap: wrap"
            >
                <span class="nowrap">Tanggal :</span>
                <CalendarPicker v-model="selectedDate" />

                <span class="nowrap">Terapis :</span>
                <select v-model="selectedProvider" style="width: 240px">
                    <option value="">Semua</option>
                    <option
                        v-for="option in providerFilterOptions"
                        :key="option.value"
                        :value="option.value"
                    >
                        {{ option.label }}
                    </option>
                </select>

                <button class="btn btn-secondary" @click="loadRows">
                    Muat Ulang
                </button>

                <span class="flex-1" />

                <button class="btn btn-primary" @click="openCreate">
                    Tambah Data
                </button>
            </div>

            <div class="alert alert-info mt-12" style="margin-bottom: 0">
                Data pada halaman ini langsung disimpan ke SQLite lokal dan
                otomatis masuk ke daftar menunggu ekspor.
            </div>
        </div>

        <div v-if="error" class="alert alert-error" style="flex-shrink: 0">
            {{ error }}
        </div>

        <div class="table-wrapper flex-1 overflow-auto">
            <table class="data-table">
                <thead>
                    <tr>
                        <th style="width: 40px">#</th>
                        <th style="width: 95px">Tanggal</th>
                        <th style="width: 80px">HN</th>
                        <th style="width: 120px">NIK</th>
                        <th style="width: 180px">Nama</th>
                        <th style="width: 110px">Hak</th>
                        <th style="width: 170px">Keluhan</th>
                        <th style="width: 180px">Tindakan</th>
                        <th style="width: 140px">Terapis</th>
                        <th style="width: 110px">Pendapatan</th>
                        <th style="width: 110px">Insentif</th>
                        <th style="width: 140px">Aksi</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(row, index) in filteredRows" :key="row.id">
                        <td class="text-sm" style="text-align: center">
                            {{ index + 1 }}
                        </td>
                        <td class="text-sm">{{ row.visit_date }}</td>
                        <td class="text-sm">{{ row.hn }}</td>
                        <td class="text-sm">{{ row.cid }}</td>
                        <td>{{ fullName(row) }}</td>
                        <td>{{ row.rights }}</td>
                        <td :title="row.symptoms">{{ row.symptoms }}</td>
                        <td :title="row.procedure">{{ row.procedure }}</td>
                        <td :title="row.therapist">{{ row.therapist }}</td>
                        <td style="text-align: right">
                            {{ row.total_revenue.toFixed(2) }}
                        </td>
                        <td
                            style="
                                text-align: right;
                                font-weight: bold;
                                color: var(--highlight);
                            "
                        >
                            {{ row.payout_amount.toFixed(2) }}
                        </td>
                        <td>
                            <div class="flex gap-8">
                                <button
                                    class="btn btn-secondary btn-sm"
                                    @click="openEdit(row)"
                                >
                                    Ubah
                                </button>
                                <button
                                    class="btn btn-danger btn-sm"
                                    @click="removeRow(row.id)"
                                >
                                    Hapus
                                </button>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="filteredRows.length === 0">
                        <td
                            colspan="12"
                            style="
                                text-align: center;
                                padding: 32px;
                                color: var(--text-gray);
                            "
                        >
                            Belum ada data untuk tanggal ini. Klik "Tambah Data"
                            untuk mulai input manual.
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div
            class="flex items-center gap-12"
            style="font-size: 12px; color: var(--text-muted); flex-shrink: 0"
        >
            <span>Data tampil: {{ filteredRows.length }}</span>
            <span>|</span>
            <span>Total pendapatan: {{ totalRevenue.toFixed(2) }}</span>
            <span>|</span>
            <span>Total insentif: {{ totalPayout.toFixed(2) }}</span>
        </div>
    </div>

    <div v-if="showForm" class="modal-overlay" @click.self="closeForm">
        <div class="modal" style="width: 880px; max-width: 96vw">
            <div class="modal__header">
                <span class="modal__title">
                    {{ editing ? "Ubah Data" : "Tambah Data" }}
                </span>
                <button class="modal__close" @click="closeForm">x</button>
            </div>

            <div class="modal__body">
                <div
                    class="form-grid"
                    style="
                        grid-template-columns: 160px minmax(0, 1fr) 160px minmax(0, 1fr);
                    "
                >
                    <label>Tanggal kunjungan :</label>
                    <CalendarPicker v-model="form.visit_date" />

                    <label>VN lokal :</label>
                    <input
                        :value="form.vn || '(akan dibuat otomatis)'"
                        type="text"
                        readonly
                    />

                    <label>HN :</label>
                    <input
                        v-model="form.hn"
                        type="text"
                        placeholder="Nomor HN"
                    />

                    <label>NIK / CID :</label>
                    <input
                        v-model="form.cid"
                        type="text"
                        placeholder="Nomor identitas"
                    />

                    <label>Nama depan :</label>
                    <input
                        v-model="form.first_name"
                        type="text"
                        placeholder="Nama depan"
                    />

                    <label>Nama belakang :</label>
                    <input
                        v-model="form.last_name"
                        type="text"
                        placeholder="Nama belakang"
                    />

                    <label>Gender :</label>
                    <select v-model="form.gender">
                        <option value="">Pilih</option>
                        <option value="L">Laki-laki</option>
                        <option value="P">Perempuan</option>
                    </select>

                    <label>Usia :</label>
                    <input
                        v-model="form.age"
                        type="number"
                        min="0"
                        placeholder="Opsional"
                    />

                    <label>Hak :</label>
                    <input
                        v-model="form.rights"
                        list="rights-options"
                        type="text"
                        placeholder="Mis. UHC"
                    />

                    <label>Terapis :</label>
                    <input
                        v-model="form.therapist"
                        list="therapist-options"
                        type="text"
                        placeholder="Nama terapis"
                    />

                    <label style="align-self: start; padding-top: 6px">
                        Keluhan :
                    </label>
                    <textarea
                        v-model="form.symptoms"
                        rows="3"
                        placeholder="Keluhan utama pasien"
                    />

                    <label>Tindakan :</label>
                    <input
                        v-model="form.procedure"
                        list="procedure-options"
                        placeholder="Pisahkan dengan koma jika lebih dari satu"
                    />

                    <label>Total pendapatan :</label>
                    <input
                        v-model="form.total_revenue"
                        type="number"
                        min="0"
                        step="0.01"
                        placeholder="0.00"
                    />

                    <label>Preset insentif :</label>
                    <select
                        v-model="selectedPayoutPreset"
                        @change="applyPayoutPreset"
                    >
                        <option value="">Pilih preset</option>
                        <option
                            v-for="option in payoutOptions"
                            :key="option.id"
                            :value="String(option.amount)"
                        >
                            {{ option.label }} ({{ option.amount.toFixed(2) }})
                        </option>
                    </select>

                    <label>Nilai insentif :</label>
                    <input
                        v-model="form.payout_amount"
                        type="number"
                        min="0"
                        step="0.01"
                        placeholder="0.00"
                    />
                </div>

                <p class="text-sm text-gray mt-12">
                    Hak, tindakan, dan terapis bisa diketik manual. Master data
                    hanya berfungsi sebagai saran pengisian.
                </p>

                <div
                    v-if="formError"
                    class="alert alert-error mt-12"
                    style="margin-bottom: 0"
                >
                    {{ formError }}
                </div>
            </div>

            <div class="modal__footer">
                <button class="btn btn-secondary" @click="closeForm">
                    Batal
                </button>
                <button class="btn btn-primary" @click="saveForm">
                    {{ editing ? "Simpan Perubahan" : "Simpan Data" }}
                </button>
            </div>
        </div>
    </div>

    <datalist id="rights-options">
        <option
            v-for="option in rightsOptions"
            :key="option.value"
            :value="option.value"
        >
            {{ option.label }}
        </option>
    </datalist>

    <datalist id="therapist-options">
        <option
            v-for="option in therapistOptions"
            :key="option.value"
            :value="option.value"
        >
            {{ option.label }}
        </option>
    </datalist>

    <datalist id="procedure-options">
        <option
            v-for="option in procedureOptions"
            :key="option.key"
            :value="option.value"
        >
            {{ option.label }}
        </option>
    </datalist>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useAppStore } from "@/stores/app";
import * as cmd from "@/composables/useCommands";
import type {
    DrugConfig,
    PayoutOption,
    PendingRow,
    ProcedureConfig,
    ProviderConfig,
    PttypeConfig,
    UpsertPendingInput,
} from "@/types";
import { useToast } from "@/composables/useToast";
import CalendarPicker from "@/components/controls/CalendarPicker.vue";

type PendingFormState = {
    id: number | null;
    original_visit_date: string;
    vn: string;
    visit_date: string;
    hn: string;
    cid: string;
    first_name: string;
    last_name: string;
    gender: string;
    age: string;
    rights: string;
    symptoms: string;
    procedure: string;
    therapist: string;
    total_revenue: string;
    payout_amount: string;
};

const store = useAppStore();
const { show: showToast } = useToast();

const selectedDate = ref(todayStr());
const selectedProvider = ref("");
const rows = ref<PendingRow[]>([]);
const payoutOptions = ref<PayoutOption[]>([]);
const providers = ref<ProviderConfig[]>([]);
const pttypes = ref<PttypeConfig[]>([]);
const procedures = ref<ProcedureConfig[]>([]);
const drugs = ref<DrugConfig[]>([]);
const error = ref("");

const showForm = ref(false);
const editing = ref(false);
const form = ref<PendingFormState>(createEmptyForm(selectedDate.value));
const selectedPayoutPreset = ref("");
const formError = ref("");

const filteredRows = computed(() => {
    if (!selectedProvider.value) return rows.value;

    return rows.value.filter((row) =>
        row.therapist
            .split(",")
            .map((name) => name.trim())
            .includes(selectedProvider.value),
    );
});

const totalRevenue = computed(() =>
    filteredRows.value.reduce((sum, row) => sum + row.total_revenue, 0),
);
const totalPayout = computed(() =>
    filteredRows.value.reduce((sum, row) => sum + row.payout_amount, 0),
);

const providerFilterOptions = computed(() => {
    const map = new Map<string, string>();

    for (const row of rows.value) {
        for (const name of row.therapist
            .split(",")
            .map((item) => item.trim())
            .filter(Boolean)) {
            map.set(name, resolveProviderLabel(name));
        }
    }

    return [...map.entries()]
        .sort((a, b) => a[1].localeCompare(b[1]))
        .map(([value, label]) => ({ value, label }));
});

const rightsOptions = computed(() =>
    pttypes.value.map((item) => ({
        value: item.short_name || item.name,
        label: item.name,
    })),
);

const therapistOptions = computed(() =>
    providers.value.map((item) => ({
        value: item.short_name || item.full_name,
        label: item.full_name,
    })),
);

const procedureOptions = computed(() => {
    const items = [
        ...procedures.value.map((item) => ({
            key: `procedure-${item.id}`,
            value: item.short_name || item.name,
            label: item.name,
        })),
        ...drugs.value.map((item) => ({
            key: `drug-${item.id}`,
            value: item.short_name || item.name,
            label: item.name,
        })),
    ];

    return items.sort((a, b) => a.value.localeCompare(b.value));
});

onMounted(async () => {
    await Promise.all([loadLookups(), loadRows()]);
});

watch(selectedDate, async () => {
    await loadRows();
});

watch(
    () => store.pendingRefreshKey,
    async () => {
        await loadRows();
    },
);

function todayStr() {
    return new Date().toISOString().slice(0, 10);
}

function createEmptyForm(date: string): PendingFormState {
    return {
        id: null,
        original_visit_date: date,
        vn: "",
        visit_date: date,
        hn: "",
        cid: "",
        first_name: "",
        last_name: "",
        gender: "",
        age: "",
        rights: "",
        symptoms: "",
        procedure: "",
        therapist: "",
        total_revenue: "",
        payout_amount: "",
    };
}

function buildLocalVn() {
    const time = Date.now().toString(36).toUpperCase();
    const random = Math.random().toString(36).slice(2, 6).toUpperCase();
    return `OFFLINE-${time}-${random}`;
}

function fullName(row: PendingRow) {
    return [row.first_name, row.last_name].filter(Boolean).join(" ");
}

function resolveProviderLabel(value: string) {
    const matched = providers.value.find(
        (item) =>
            item.short_name === value ||
            item.full_name === value ||
            String(item.health_med_provider_id) === value,
    );
    return matched?.short_name || matched?.full_name || value;
}

async function loadLookups() {
    const results = await Promise.allSettled([
        cmd.getPayoutOptions(),
        cmd.getAllProviders(),
        cmd.getAllPttypes(),
        cmd.getAllProcedures(),
        cmd.getAllDrugs(),
    ]);

    if (results[0].status === "fulfilled") {
        payoutOptions.value = results[0].value;
    }
    if (results[1].status === "fulfilled") {
        providers.value = results[1].value;
    }
    if (results[2].status === "fulfilled") {
        pttypes.value = results[2].value;
    }
    if (results[3].status === "fulfilled") {
        procedures.value = results[3].value;
    }
    if (results[4].status === "fulfilled") {
        drugs.value = results[4].value;
    }
}

async function loadRows() {
    error.value = "";

    try {
        rows.value = await cmd.getPendingExport(
            selectedDate.value,
            selectedDate.value,
        );
    } catch (err: any) {
        error.value = String(err);
    }
}

async function openCreate() {
    await loadLookups();
    editing.value = false;
    selectedPayoutPreset.value = "";
    formError.value = "";
    form.value = createEmptyForm(selectedDate.value);
    showForm.value = true;
}

async function openEdit(row: PendingRow) {
    await loadLookups();
    editing.value = true;
    formError.value = "";
    form.value = {
        id: row.id,
        original_visit_date: row.visit_date,
        vn: row.vn,
        visit_date: row.visit_date,
        hn: row.hn,
        cid: row.cid,
        first_name: row.first_name,
        last_name: row.last_name,
        gender: row.gender,
        age: row.age == null ? "" : String(row.age),
        rights: row.rights,
        symptoms: row.symptoms,
        procedure: row.procedure,
        therapist: row.therapist,
        total_revenue: String(row.total_revenue),
        payout_amount: String(row.payout_amount),
    };
    selectedPayoutPreset.value = payoutOptions.value.some(
        (option) => option.amount === row.payout_amount,
    )
        ? String(row.payout_amount)
        : "";
    showForm.value = true;
}

function closeForm() {
    showForm.value = false;
    formError.value = "";
}

function applyPayoutPreset() {
    if (!selectedPayoutPreset.value) return;
    form.value.payout_amount = selectedPayoutPreset.value;
}

function normalizeProcedure(value: string) {
    return value
        .split(",")
        .map((item) => item.trim())
        .filter(Boolean)
        .join(", ");
}

function parseOptionalInteger(value: string) {
    const trimmed = value.trim();
    if (!trimmed) return null;
    const parsed = Number(trimmed);
    return Number.isInteger(parsed) && parsed >= 0 ? parsed : NaN;
}

function parseRequiredNumber(value: string) {
    const parsed = Number(value);
    return Number.isFinite(parsed) && parsed >= 0 ? parsed : NaN;
}

async function saveForm() {
    formError.value = "";

    const firstName = form.value.first_name.trim();
    const lastName = form.value.last_name.trim();
    const therapist = form.value.therapist.trim();
    const rights = form.value.rights.trim();
    const visitDate = form.value.visit_date;
    const age = parseOptionalInteger(form.value.age);
    const totalRevenue = parseRequiredNumber(form.value.total_revenue);
    const payoutAmount = parseRequiredNumber(form.value.payout_amount);

    if (!visitDate) {
        formError.value = "Tanggal kunjungan wajib diisi.";
        return;
    }

    if (!firstName && !lastName) {
        formError.value = "Isi minimal nama depan atau nama belakang.";
        return;
    }

    if (!rights) {
        formError.value = "Hak wajib diisi.";
        return;
    }

    if (!therapist) {
        formError.value = "Terapis wajib diisi.";
        return;
    }

    if (Number.isNaN(age)) {
        formError.value = "Usia harus berupa angka bulat nol atau lebih.";
        return;
    }

    if (Number.isNaN(totalRevenue)) {
        formError.value = "Total pendapatan harus berupa angka nol atau lebih.";
        return;
    }

    if (Number.isNaN(payoutAmount)) {
        formError.value = "Nilai insentif harus berupa angka nol atau lebih.";
        return;
    }

    const record: UpsertPendingInput = {
        visit_date: visitDate,
        vn: form.value.vn || buildLocalVn(),
        hn: form.value.hn.trim(),
        cid: form.value.cid.trim(),
        first_name: firstName,
        last_name: lastName,
        gender: form.value.gender,
        age: age === null ? null : age,
        rights,
        symptoms: form.value.symptoms.trim(),
        procedure: normalizeProcedure(form.value.procedure),
        therapist,
        total_revenue: totalRevenue,
        payout_amount: payoutAmount,
    };

    try {
        await cmd.upsertPending(record);

        if (
            editing.value &&
            form.value.id != null &&
            form.value.original_visit_date !== record.visit_date
        ) {
            await cmd.deletePendingById(form.value.id);
        }

        showToast(
            editing.value
                ? "Perubahan data berhasil disimpan."
                : "Data berhasil ditambahkan.",
            "success",
        );
        selectedDate.value = record.visit_date;
        showForm.value = false;
        await loadRows();
        store.triggerPendingRefresh();
    } catch (err: any) {
        formError.value = String(err);
    }
}

async function removeRow(id: number) {
    if (!confirm("Hapus data ini dari penyimpanan lokal?")) return;

    error.value = "";

    try {
        await cmd.deletePendingById(id);
        showToast("Data berhasil dihapus.", "success");
        await loadRows();
        store.triggerPendingRefresh();
    } catch (err: any) {
        error.value = String(err);
    }
}
</script>

<style scoped>
.data-table td[title] {
    cursor: help;
    text-decoration: underline dotted rgba(0, 0, 0, 0.15);
    text-underline-offset: 2px;
}
</style>
