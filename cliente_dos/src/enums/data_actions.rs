use std::sync::mpsc::Sender;
pub struct AddData {
    data: (u8, u8)
}

impl AddData {
    pub fn init(data: (u8, u8)) -> AddData {
        AddData {
            data
        }
    }

    pub fn get_data(&self) -> (u8, u8) {
        self.data.clone()
    }
}

pub struct GetData {
    sender: Sender<Vec<(u8, u8)>>
}

impl GetData {
    pub fn init(sender: Sender<Vec<(u8, u8)>>) -> GetData {
        GetData {
            sender
        }
    }

    pub fn send_data(&self, data: Vec<(u8, u8)>) {
        self.sender.send(data).unwrap();
    }
}


pub enum DataAction {
    Add(AddData),
    Get(GetData),
}
