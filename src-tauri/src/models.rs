use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PttypeConfig {
    pub id: i64,
    pub pttype: String,
    pub name: String,
    pub pcode: String,
    pub hipdata_code: String,
    pub short_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureConfig {
    pub id: i64,
    pub icode: String,
    pub name: String,
    pub short_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugConfig {
    pub id: i64,
    pub icode: String,
    pub name: String,
    pub short_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: i64,
    pub health_med_provider_id: i64,
    pub full_name: String,
    pub short_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutOption {
    pub id: i64,
    pub amount: f64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingRow {
    pub id: i64,
    pub visit_date: String,
    pub vn: String,
    pub hn: String,
    pub cid: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub age: Option<i64>,
    pub rights: String,
    pub symptoms: String,
    pub procedure: String,
    pub therapist: String,
    pub total_revenue: f64,
    pub payout_amount: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub month: String,
    pub count: i64,
    pub total_revenue: f64,
    pub total_payout: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertPendingInput {
    pub visit_date: String,
    pub vn: String,
    pub hn: String,
    pub cid: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub age: Option<i64>,
    pub rights: String,
    pub symptoms: String,
    pub procedure: String,
    pub therapist: String,
    pub total_revenue: f64,
    pub payout_amount: f64,
}
