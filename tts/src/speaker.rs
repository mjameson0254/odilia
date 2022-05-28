use ssip_client::{
    client::Client,
    fifo::{self, Builder},
    ClientError, ClientName,
};
use tracing::{debug, trace, warn};

pub struct Speaker {
    client: Client<fifo::UnixStream>,
}
impl Speaker {
    ///initialises a `Speaker` object
    #[tracing::instrument]
    pub fn new(name: &str) -> Result<Self, ClientError> {
        debug!("initialising speech dispatcher client");
        let mut client = Builder::new().build()?;
        debug!(name=%name, "setting name and user");
        client
            .set_client_name(ClientName::new(name, name))?
            .check_client_name_set()?;
        debug!("ssip client initialisation successfull");
        Ok(Self { client })
    }
    ///speaks the given message
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn speak(&mut self, message: &str) -> Result<(), ClientError> {
        trace!(message=%message, "speaking message");
        let msg_id = self
            .client
            .speak()?
            .send_line(message)?
            .receive_message_id()?;
        trace!(id=%msg_id, "speech started successfully");
        Ok(())
    }
}

impl Drop for Speaker {
    #[tracing::instrument(skip(self))]
    fn drop(&mut self) {
        debug!("destroying connection to speech dispatcher socket");
        self.client.quit().unwrap();
    }
}
