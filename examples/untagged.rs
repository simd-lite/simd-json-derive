use simd_json::Node;
use simd_json_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse<'de> {
    pub foo: &'de str,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse<'de> {
    pub error: &'de str,
}

#[derive(Debug)]
enum Response<'de> {
    LoginResponse(LoginResponse<'de>),
    Error(ErrorResponse<'de>),
}

fn parse(data: &mut [u8]) -> Result<Response, Box<dyn std::error::Error>> {
    let tape = simd_json::to_tape(data)?;

    if let [Node::Object { len: 1, count: 2 }, Node::String("error"), Node::String(error)] =
        tape.0.as_slice()
    {
        Ok(Response::Error(ErrorResponse { error }))
    } else {
        let mut itr = tape.0.into_iter().peekable();
        Ok(Response::LoginResponse(LoginResponse::from_tape(&mut itr)?))
    }
}

fn main() {
    let mut data = br#"{"error":"hello world!"}"#.to_vec();
    println!("{:#?}", parse(&mut data))
}
