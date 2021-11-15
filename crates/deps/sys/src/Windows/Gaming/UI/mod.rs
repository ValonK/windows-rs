#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
}
impl ::core::marker::Copy for GameChatMessageOrigin {}
impl ::core::clone::Clone for GameChatMessageOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameChatMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameChatMessageReceivedEventArgs {}
impl ::core::clone::Clone for GameChatMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameChatOverlay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameChatOverlay {}
impl ::core::clone::Clone for GameChatOverlay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameChatOverlayMessageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameChatOverlayMessageSource {}
impl ::core::clone::Clone for GameChatOverlayMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: Self = Self(0i32);
    pub const BottomLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const MiddleRight: Self = Self(3i32);
    pub const MiddleLeft: Self = Self(4i32);
    pub const TopCenter: Self = Self(5i32);
    pub const TopLeft: Self = Self(6i32);
    pub const TopRight: Self = Self(7i32);
}
impl ::core::marker::Copy for GameChatOverlayPosition {}
impl ::core::clone::Clone for GameChatOverlayPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameUIProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameUIProviderActivatedEventArgs {}
impl ::core::clone::Clone for GameUIProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarStatics {}
impl ::core::clone::Clone for IGameBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameChatMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameChatMessageReceivedEventArgs {}
impl ::core::clone::Clone for IGameChatMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameChatOverlay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameChatOverlay {}
impl ::core::clone::Clone for IGameChatOverlay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameChatOverlayMessageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameChatOverlayMessageSource {}
impl ::core::clone::Clone for IGameChatOverlayMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameChatOverlayStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameChatOverlayStatics {}
impl ::core::clone::Clone for IGameChatOverlayStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameUIProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameUIProviderActivatedEventArgs {}
impl ::core::clone::Clone for IGameUIProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}