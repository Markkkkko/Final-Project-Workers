

use serde::Deserialize; //debug tells to deserialize
#[allow(dead_code)] //debug error
#[derive(Debug, Deserialize, Clone)]
pub struct Employee { //create struct holding entire data
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub department: String,
    pub salary: u32,
    #[serde(rename = "Joining Date")]
    pub joining_date: String,
    #[serde(rename = "Performance Score")]
    pub performance_score: Option<f64>,
    pub experience: u32,
    pub status: String,
    pub location: String,
    pub session: String,
}
