#![cfg(feature = "share_clipboard")]
use clipboard_ext::prelude::*;
use clipboard_ext::x11_bin::ClipboardContext;
use std::error::Error;

pub fn read_share() -> Result<String, Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;

    #[cfg(feature = "share_selection")]
    return ctx.get_contents().map_err(Into::into);

    #[cfg(not(feature = "share_selection"))]
    return ctx.get_contents().map_err(Into::into);
}

pub fn write_share(url: String) -> Result<(), Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;

    #[cfg(feature = "share_selection")]
    return ctx.set_contents(url).map_err(Into::into);

    #[cfg(not(feature = "share_selection"))]
    return ctx.set_contents(url).map_err(Into::into);
}
