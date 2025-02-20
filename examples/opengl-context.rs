use sfml::{SfResult, window::Context};

fn main() -> SfResult<()> {
    // We have no active context, so this should be null
    assert_eq!(Context::active_context(), std::ptr::null());
    {
        // Let's create a context
        let mut ctx = Context::new()?;
        // I guess the active context should now be the same as what we have
        assert_eq!(Context::active_context(), &*ctx);
        eprintln!("Active context id: {}", Context::active_context_id());
        let settings = ctx.settings();
        eprintln!("Settings of created context: {settings:#?}");
        ctx.set_active(false)?;
        // Deactivated context, so this should be null
        assert_eq!(Context::active_context(), std::ptr::null());
        eprintln!("Active context id: {}", Context::active_context_id());
        ctx.set_active(true)?;
        // Activated once again
        assert_eq!(Context::active_context(), &*ctx);
        eprintln!("Active context id: {}", Context::active_context_id());
    }
    // Once again, no active context, so this should be null
    assert_eq!(Context::active_context(), std::ptr::null());
    // Try to query a random OpenGL function
    dbg!(Context::get_function(c"glWhatever"));
    eprintln!("Okay then... Goodbye!");
    Ok(())
}
