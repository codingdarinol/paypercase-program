export interface PttypeConfig {
    id: number;
    pttype: string;
    name: string;
    pcode: string;
    hipdata_code: string;
    short_name: string;
}

export interface ProcedureConfig {
    id: number;
    icode: string;
    name: string;
    short_name: string;
}

export interface DrugConfig {
    id: number;
    icode: string;
    name: string;
    short_name: string;
}

export interface ProviderConfig {
    id: number;
    health_med_provider_id: number;
    full_name: string;
    short_name: string;
}

export interface PayoutOption {
    id: number;
    amount: number;
    label: string;
}

export interface PendingRow {
    id: number;
    visit_date: string;
    vn: string;
    hn: string;
    cid: string;
    first_name: string;
    last_name: string;
    gender: string;
    age: number | null;
    rights: string;
    symptoms: string;
    procedure: string;
    therapist: string;
    total_revenue: number;
    payout_amount: number;
    created_at: string;
}

export interface UpsertPendingInput {
    visit_date: string;
    vn: string;
    hn: string;
    cid: string;
    first_name: string;
    last_name: string;
    gender: string;
    age: number | null;
    rights: string;
    symptoms: string;
    procedure: string;
    therapist: string;
    total_revenue: number;
    payout_amount: number;
}

export interface MonthlyStats {
    month: string;
    count: number;
    total_revenue: number;
    total_payout: number;
}
