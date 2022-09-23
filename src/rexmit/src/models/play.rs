
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Play {
    pub id: String,
    pub href: String,
    pub initially_played: u64,
    pub last_played: u64
}