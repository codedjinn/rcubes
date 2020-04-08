
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};

use web_sys::{XmlHttpRequest};

use crate::*;

pub fn get(url: String) -> Result<(), js_sys::Error> {
    // TODO: use async/wait or have a closure parameter
    let xhr = XmlHttpRequest::new()?;
    xhr.open_with_async(&"GET", &url, true)?;
    xhr.send()?;
    
    let event_target = &xhr as &web_sys::EventTarget;

    let c_xhr = xhr.clone();
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        // stats() = Result<u16, JsValue>
        match c_xhr.status() {
            Ok(value) => {
                if value == 200 {
                    // response_text() = Result<Option<string>, JsValue>
                    match c_xhr.response_text() {
                        Ok(result) => console_log!("{}", result.unwrap()),
                        Err(e) => console_log!("{:?}", e)
                    }
                }
            },
            Err(e) => console_log!("Error: {:?}", e)
        }
    }));
    
    event_target.add_event_listener_with_callback("readystatechange", closure.as_ref().unchecked_ref())?;
    closure.forget();

    return Ok(());
}

// pub async fn get(uri: String) -> String {

//     console_log!("[get] - entering");

//     let mut options = RequestInit::new();
//     options.method("GET");
//     options.mode(RequestMode::Cors);

//     let request = Request::new_with_str_and_init(&uri, &options).unwrap();

//     request.headers().set("Accept", "text/json").unwrap();

//     let result: Option<&str> = None;

//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

//     assert!(resp_value.is_instance_of::<Response>());

//     console_log!("[get] - exiting");

//     String::from("SUCCESS")
// }
// // pub async fn get(uri: String) -> Result<JsValue, JsValue> {
//     let mut options = RequestInit::new();
//     options.method("GET");
//     options.mode(RequestMode::Cors);

//     let request = Request::new_with_str_and_init(&uri, &options)?;

//     request
//         .headers()
//         .set("Accept", "text/json")?;

//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

//     let resp: Response = resp_value.dyn_into().unwrap();

//     let json = JsFuture::from(resp.json()?).await?;

//     Ok(JsValue::from_str("pew"))
// }