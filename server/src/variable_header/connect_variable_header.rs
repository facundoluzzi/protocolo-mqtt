pub fn get_keep_alive(variable_header: &[u8]) -> Option<u8> {
    let msb_keep_alive = variable_header[8];
    let lsb_keep_alive = variable_header[9];

    if msb_keep_alive == 0x00 && lsb_keep_alive == 0x00 {
        None
    } else if msb_keep_alive > lsb_keep_alive {
        Some(msb_keep_alive)
    } else {
        Some(lsb_keep_alive)
    }
}

pub fn check_variable_header_len(variable_header: &[u8]) -> Result<String, String> {
    if variable_header.len() != 10 {
        return Err("Invalid variable header length".to_string());
    } else {
        match check_mqtt(variable_header){
            Ok(check) => return Ok(check),
            Err(err) => return Err(err.to_string()),
        };
    }
}

pub fn check_mqtt(variable_header: &[u8]) -> Result<String, String> {
    let vec = [0x00, 0x04, 0x4D, 0x51, 0x54, 0x54,];
    let mut violation = false;
    let mut counter: usize = 0;
    for i in vec {
        if variable_header[counter] != i {
            violation = true;
        }
        counter = counter + 1;
    }

    if violation {
        return Err("Invalid MQTT".to_string());
    } else {
        return Ok("OK".to_string());
    };
}
