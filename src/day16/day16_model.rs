// model types for Day16

#[derive(Clone)]
pub struct Room{
    pub valve: String,
    pub flow_rate:i32,
    pub leads_to: Vec<String>
}

