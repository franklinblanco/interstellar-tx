use actix_web::Responder;

/// The point of implementing this is to let you use your own custom return types. For my use case, I have a TypedHttpResponse that implements responder.
pub trait HeaderAppending: Responder {
    fn append_header(self) -> Self;
}