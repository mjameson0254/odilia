use ssip_client::{
    client::Client,
    fifo::{self, Builder},
    ClientError, ClientName,
};

pub struct Speaker {
    client: Client<fifo::UnixStream>,
}
impl Speaker {
    ///initialises a `Speaker` object
    pub fn new(name: &str) -> Result<Self, ClientError> {
        let mut client = Builder::new().build()?;
        client
            .set_client_name(ClientName::new(name, name))?
            .check_client_name_set()?;
        Ok(Self { client })
    }
    ///speaks the given message
    pub fn speak(&mut self, message: &str) -> Result<(), ClientError> {
        let _msg_id = self
            .client
            .speak()?
            .send_line(message)?
            .receive_message_id()?;
        Ok(())
    }
}

impl Drop for Speaker {
    fn drop(&mut self) {
        self.client.quit().unwrap();
    }
}
