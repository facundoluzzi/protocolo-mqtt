use std::sync::mpsc::Sender;
pub struct AddData {
    data: String,
}

impl AddData {
    pub fn init(data: String) -> AddData {
        AddData { data }
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}

pub struct GetData {
    sender: Sender<Vec<String>>,
}

impl GetData {
    pub fn init(sender: Sender<Vec<String>>) -> GetData {
        GetData { sender }
    }

    pub fn send_data(&self, data: Vec<String>) {
        self.sender.send(data).unwrap();
    }
}

pub enum DataAction {
    Add(AddData),
    Get(GetData),
}
