mod SendMessage;
mod dispatch_message;
mod get_async_key_state;
mod peek_message;
pub use dispatch_message::{
    hook_DispatchMessageA, hook_DispatchMessageW, DispatchMessageADetour, DispatchMessageWDetour,
};
pub use get_async_key_state::{hook_GetAsyncKeyState, GetAsyncKeyStateDetour};
pub use peek_message::{
    hook_PeekMessageA, hook_PeekMessageW, PeekMessageADetour, PeekMessageWDetour,
};
pub use SendMessage::{hook_SendMessageA, SendMessageADetour};
