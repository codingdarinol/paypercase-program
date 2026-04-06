import { invoke } from "@tauri-apps/api/core";
import type {
    PttypeConfig,
    ProcedureConfig,
    DrugConfig,
    ProviderConfig,
    PayoutOption,
    PendingRow,
    UpsertPendingInput,
    MonthlyStats,
} from "@/types";

export const getAllPttypes = () => invoke<PttypeConfig[]>("get_all_pttypes");

export const savePttype = (
    pttype: string,
    name: string,
    pcode: string,
    hipdataCode: string,
    shortName: string,
) =>
    invoke<void>("save_pttype", {
        pttype,
        name,
        pcode,
        hipdataCode,
        shortName,
    });

export const deletePttype = (id: number) =>
    invoke<void>("delete_pttype", { id });

export const getAllProcedures = () =>
    invoke<ProcedureConfig[]>("get_all_procedures");

export const saveProcedure = (icode: string, name: string, shortName: string) =>
    invoke<void>("save_procedure", { icode, name, shortName });

export const deleteProcedure = (id: number) =>
    invoke<void>("delete_procedure", { id });

export const getAllDrugs = () => invoke<DrugConfig[]>("get_all_drugs");

export const saveDrug = (icode: string, name: string, shortName: string) =>
    invoke<void>("save_drug", { icode, name, shortName });

export const deleteDrug = (id: number) => invoke<void>("delete_drug", { id });

export const getAllProviders = () =>
    invoke<ProviderConfig[]>("get_all_providers");

export const saveProvider = (
    healthMedProviderId: number,
    fullName: string,
    shortName: string,
) =>
    invoke<void>("save_provider", {
        healthMedProviderId,
        fullName,
        shortName,
    });

export const deleteProvider = (id: number) =>
    invoke<void>("delete_provider", { id });

export const getPayoutOptions = () =>
    invoke<PayoutOption[]>("get_payout_options");

export const addPayoutOption = (amount: number, label: string) =>
    invoke<void>("add_payout_option", { amount, label });

export const deletePayoutOption = (id: number) =>
    invoke<void>("delete_payout_option", { id });

export const upsertPending = (record: UpsertPendingInput) =>
    invoke<void>("upsert_pending", { record });

export const getPendingExport = (dateFrom: string, dateTo: string) =>
    invoke<PendingRow[]>("get_pending_export", { dateFrom, dateTo });

export const deletePendingById = (id: number) =>
    invoke<void>("delete_pending_by_id", { id });

export const exportPendingCsv = (
    dateFrom: string,
    dateTo: string,
    filePath: string,
) => invoke<string>("export_pending_csv", { dateFrom, dateTo, filePath });

export const previewDeleteRange = (dateFrom: string, dateTo: string) =>
    invoke<PendingRow[]>("preview_delete_range", { dateFrom, dateTo });

export const deletePendingRange = (dateFrom: string, dateTo: string) =>
    invoke<number>("delete_pending_range", { dateFrom, dateTo });

export const getMonthlyStats = () =>
    invoke<MonthlyStats[]>("get_monthly_stats");
