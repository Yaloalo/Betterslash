use nvim_oxi::api::{self, create_user_command};

#[nvim_oxi::module]
fn betterslash() -> nvim_oxi::Result<()> {
    create_user_command("HighlightDemo", |_args| {
        let mut buf = api::get_current_buf();  // <- make it mutable
        let ns_id = api::create_namespace("betterslash");

        buf.clear_namespace(ns_id, 0..)?;

        for line in 0..5 {
            buf.add_highlight(ns_id, "Search", line, 0..)?;
        }

        Ok(())
    }, &Default::default())?;

    Ok(())
}
