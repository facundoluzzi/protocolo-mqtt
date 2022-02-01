use std::sync::mpsc::Sender;
pub struct AddData {
    data: (i32, i32),
}

impl AddData {
    pub fn init(data: (i32, i32)) -> AddData {
        AddData { data }
    }

    pub fn get_data(&self) -> (i32, i32) {
        self.data.clone()
    }
}

pub struct GetData {
    sender: Sender<Vec<(i32, i32)>>,
}

impl GetData {
    pub fn init(sender: Sender<Vec<(i32, i32)>>) -> GetData {
        GetData { sender }
    }

    pub fn send_data(&self, data: Vec<(i32,i32)>) {
        self.sender.send(data).unwrap();
    }
}

pub enum DataAction {
    Add(AddData),
    Get(GetData),
}
