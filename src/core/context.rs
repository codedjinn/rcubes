
// // will figure out how to do this better later

// use std::sync::Once;

// use web_sys::WebGlRenderingContext;

// #[derive(Clone)]
// pub struct GameContext {
//     gl: &'static WebGlRenderingContext
// }

// impl GameContext {

//     pub fn init(context: &WebGlRenderingContext) {

//         let instance= GameContext {
//             gl: &'static context
//         };

//         CONTEXT = Some(instance);
//     }

//     pub fn gl() -> WebGlRenderingContext {
        
//     }
// }

// static CONTEXT: Option<GameContext> = None;
// static START: Once = Once::new();



// // static CONTEXT_INITIALIZED: bool = false;
// // static SINGLETON: Option<GameContext> = None;

// // impl GameContext {

// //     // should only call once
// //     pub fn init(context: WebGlRenderingContext) {
        
// //         if !CONTEXT_INITIALIZED {

// //             static INIT:Once = Once::new();

// //             INIT.call_once(|| {
// //                 SINGLETON = Some(GameContext {
// //                     gl: context 
// //                 });
// //             });

// //         } else {
            
// //             // log to console

// //         }
// //     }

// //     pub fn get() -> Option<GameContext> {
// //         SINGLETON.clone()
// //     }

// // }