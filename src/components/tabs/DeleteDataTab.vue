<template>
    <div class="flex-col h-full" style="gap: 16px; padding: 16px">
        <!-- Delete by date range -->
        <div class="group-box">
            <span class="group-box__title">🗑️ Hapus Data Berdasarkan Rentang Tanggal</span>
            <div style="padding-top: 12px">
                <div
                    class="flex items-center gap-12 mb-12"
                    style="flex-wrap: wrap"
                >
                    <span class="nowrap">Tanggal mulai :</span>
                    <CalendarPicker v-model="dateFrom" />
                    <span class="nowrap">Tanggal akhir :</span>
                    <CalendarPicker v-model="dateTo" />
                </div>

                <div
                    v-if="rangeInfo"
                    class="alert"
                    :class="
                        rangeInfo.type === 'info'
                            ? 'alert-info'
                            : 'alert-warning'
                    "
                    style="margin-bottom: 10px"
                >
                    {{ rangeInfo.text }}
                </div>

                <div class="flex items-center gap-8">
                    <button class="btn btn-secondary" @click="previewRange">
                        🔍 Pratinjau Data yang Akan Dihapus
                    </button>
                    <button
                        class="btn btn-danger"
                        :disabled="previewRows.length === 0"
                        @click="deleteRange"
                    >
                        🗑️ Hapus Data pada Rentang Ini
                    </button>
                </div>

                <!-- Preview table -->
                <div
                    v-if="previewRows.length > 0"
                    class="table-wrapper"
                    style="max-height: 200px; margin-top: 12px"
                >
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>#</th>
                                <th>Tanggal</th>
                                <th>VN</th>
                                <th>HN</th>
                                <th>Nama Lengkap</th>
                                <th>Terapis</th>
                                <th>Insentif (฿)</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(r, i) in previewRows" :key="r.id">
                                <td>{{ i + 1 }}</td>
                                <td>{{ r.visit_date }}</td>
                                <td>{{ r.vn }}</td>
                                <td>{{ r.hn }}</td>
                                <td>{{ r.first_name }} {{ r.last_name }}</td>
                                <td>{{ r.therapist }}</td>
                                <td style="text-align: right">
                                    {{ r.payout_amount.toFixed(2) }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>

        <!-- Alert -->
        <div v-if="error" class="alert alert-error">⚠️ {{ error }}</div>

        <!-- Monthly stats -->
        <div class="group-box flex-col" style="flex: 1; min-height: 0">
            <span class="group-box__title">📊 Statistik Bulanan</span>
            <div class="flex items-center gap-8 mb-8" style="padding-top: 12px">
                <button class="btn btn-secondary btn-sm" @click="loadStats">
                    🔄 Muat Ulang
                </button>
            </div>
            <div class="table-wrapper flex-1 overflow-auto">
                <table class="data-table">
                    <thead>
                        <tr>
                            <th class="sortable" @click="sortStats('month')">
                                Bulan {{ sortIcon("month") }}
                            </th>
                            <th class="sortable" @click="sortStats('count')">
                                Jumlah (data) {{ sortIcon("count") }}
                            </th>
                            <th
                                class="sortable"
                                @click="sortStats('total_revenue')"
                            >
                                Total Pendapatan (฿) {{ sortIcon("total_revenue") }}
                            </th>
                            <th
                                class="sortable"
                                @click="sortStats('total_payout')"
                            >
                                Total Insentif (฿) {{ sortIcon("total_payout") }}
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="s in sortedStats" :key="s.month">
                            <td>{{ s.month }}</td>
                            <td style="text-align: right">{{ s.count }}</td>
                            <td style="text-align: right">
                                {{ s.total_revenue.toFixed(2) }}
                            </td>
                            <td
                                style="
                                    text-align: right;
                                    font-weight: bold;
                                    color: var(--highlight);
                                "
                            >
                                {{ s.total_payout.toFixed(2) }}
                            </td>
                        </tr>
                        <tr v-if="stats.length === 0">
                            <td
                                colspan="4"
                                style="
                                    text-align: center;
                                    padding: 24px;
                                    color: var(--text-gray);
                                "
                            >
                                Tidak ada data
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import * as cmd from "@/composables/useCommands";
import type { PendingRow, MonthlyStats } from "@/types";
import { useToast } from "@/composables/useToast";
import CalendarPicker from "@/components/controls/CalendarPicker.vue";

const { show: showToast } = useToast();
const emit = defineEmits<{ "data-deleted": [] }>();

const dateFrom = ref(todayStr());
const dateTo = ref(todayStr());
const previewRows = ref<PendingRow[]>([]);
const rangeInfo = ref<{ type: string; text: string } | null>(null);
const stats = ref<MonthlyStats[]>([]);
const error = ref("");
const sortCol = ref<keyof MonthlyStats>("month");
const sortAsc = ref(false);

function todayStr() {
    return new Date().toISOString().slice(0, 10);
}

const sortedStats = computed(() => {
    return [...stats.value].sort((a, b) => {
        const av = a[sortCol.value] as any;
        const bv = b[sortCol.value] as any;
        return sortAsc.value ? (av > bv ? 1 : -1) : av < bv ? 1 : -1;
    });
});

function sortStats(col: keyof MonthlyStats) {
    if (sortCol.value === col) sortAsc.value = !sortAsc.value;
    else {
        sortCol.value = col;
        sortAsc.value = false;
    }
}

function sortIcon(col: string) {
    if (sortCol.value !== col) return "";
    return sortAsc.value ? "▲" : "▼";
}

onMounted(loadStats);

async function loadStats() {
    try {
        stats.value = await cmd.getMonthlyStats();
    } catch (e: any) {
        error.value = String(e);
    }
}

async function previewRange() {
    error.value = "";
    rangeInfo.value = null;
    try {
        previewRows.value = await cmd.previewDeleteRange(
            dateFrom.value,
            dateTo.value,
        );
        rangeInfo.value =
            previewRows.value.length > 0
                ? {
                      type: "warning",
                      text: `Ditemukan ${previewRows.value.length} data yang akan dihapus pada rentang tanggal ini`,
                  }
                : { type: "info", text: "Tidak ada data pada rentang tanggal yang dipilih" };
    } catch (e: any) {
        error.value = String(e);
    }
}

async function deleteRange() {
    if (
        !confirm(
            `Hapus ${previewRows.value.length} data pada rentang ${dateFrom.value} sampai ${dateTo.value}?\n\nTindakan ini tidak dapat dibatalkan.`,
        )
    )
        return;
    error.value = "";
    try {
        const n = await cmd.deletePendingRange(dateFrom.value, dateTo.value);
        showToast(`Berhasil menghapus ${n} data`, "success");
        previewRows.value = [];
        rangeInfo.value = null;
        await loadStats();
        emit("data-deleted");
    } catch (e: any) {
        error.value = String(e);
    }
}
</script>
