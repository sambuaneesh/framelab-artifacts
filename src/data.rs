use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    #[serde(rename = "Case Study")]
    pub case_study: String,
    #[serde(rename = "Tool")]
    pub tool: String,
    #[serde(rename = "Variant")]
    pub variant: String,
    #[serde(rename = "#Partitions")]
    pub partitions: f64,
    #[serde(rename = "Partition Size Min")]
    pub partition_size_min: f64,
    #[serde(rename = "Partition Size Med")]
    pub partition_size_med: f64,
    #[serde(rename = "Partition Size Max")]
    pub partition_size_max: f64,
    #[serde(rename = "# Obs. Ents. (%)")]
    pub obs_ents: String,
    #[serde(rename = "CiD")]
    pub cid: f64,
    #[serde(rename = "CMod")]
    pub cmod: f64,
    #[serde(rename = "BCP")]
    pub bcp: f64,
    #[serde(rename = "DI")]
    pub di: f64,
    #[serde(rename = "DTP")]
    pub dtp: f64,
    #[serde(rename = "TC")]
    pub tc: f64,
    #[serde(rename = "LC")]
    pub lc: f64,
    #[serde(rename = "MoJoFM")]
    pub mojofm: f64,
    #[serde(rename = "c2c_cvg 10%")]
    pub c2c_cvg_10: f64,
    #[serde(rename = "c2c_cvg 33%")]
    pub c2c_cvg_33: f64,
    #[serde(rename = "c2c_cvg 50%")]
    pub c2c_cvg_50: f64,
}

impl DataPoint {
    pub fn get_metric(&self, metric: &str) -> f64 {
        match metric {
            "partitions" => self.partitions,
            "partition_size_min" => self.partition_size_min,
            "partition_size_med" => self.partition_size_med,
            "partition_size_max" => self.partition_size_max,
            "cid" => self.cid,
            "cmod" => self.cmod,
            "bcp" => self.bcp,
            "di" => self.di,
            "dtp" => self.dtp,
            "tc" => self.tc,
            "lc" => self.lc,
            "mojofm" => self.mojofm,
            "c2c_cvg_10" => self.c2c_cvg_10,
            "c2c_cvg_33" => self.c2c_cvg_33,
            "c2c_cvg_50" => self.c2c_cvg_50,
            _ => 0.0,
        }
    }

    pub fn tool_variant(&self) -> String {
        format!("{} ({})", self.tool, self.variant)
    }

    pub fn get_color(&self) -> &'static str {
        match self.tool.as_str() {
            "CHUNKING" => "#3b82f6",      // Blue
            "DATACENTRIC" => "#8b5cf6",   // Purple
            "GROUND_TRUTH" => "#10b981",  // Green
            "HyDec" => "#f59e0b",         // Amber
            "LOG2MS" => "#ef4444",        // Red
            "MEM-CMT" => "#ec4899",       // Pink
            "MEM-CNTR" => "#14b8a6",      // Teal
            "MONO2MICRO" => "#f97316",    // Orange
            "ZEROSHOT" => "#6366f1",      // Indigo
            _ => "#64748b",               // Slate (default)
        }
    }
}

pub static METRICS: &[(&str, &str)] = &[
    ("partitions", "#Partitions"),
    ("partition_size_min", "Partition Size Min"),
    ("partition_size_med", "Partition Size Med"),
    ("partition_size_max", "Partition Size Max"),
    ("cid", "CiD"),
    ("cmod", "CMod"),
    ("bcp", "BCP"),
    ("di", "DI"),
    ("dtp", "DTP"),
    ("tc", "TC"),
    ("lc", "LC"),
    ("mojofm", "MoJoFM"),
    ("c2c_cvg_10", "c2c_cvg 10%"),
    ("c2c_cvg_33", "c2c_cvg 33%"),
    ("c2c_cvg_50", "c2c_cvg 50%"),
];

pub static CASE_STUDIES: &[&str] = &["7ep-demo", "JPetStore", "PartsUnlimitedMRP", "Spring-PetClinic"];
