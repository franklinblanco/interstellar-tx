use actix_web::Responder;
use uuid::Uuid;

use crate::RESPONSE_HEADER_UUID_KEY;


/// This macro is what you would typically use at the end of any service method. After your last use of Transaction, inside of a method that returns an actix web route return type. E.G. `impl Responder`
#[macro_export]
macro_rules! interstellar {
    () => {
        
    };
}

fn add_header_to_responder(responder: impl Responder, uuid: Uuid) {
    responder.customize().append_header((RESPONSE_HEADER_UUID_KEY, uuid.to_string()));
}