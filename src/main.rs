use mlua::Lua;
use std::fs;

fn main() -> mlua::Result<()> {
    let lua = Lua::new();

    lua.load(r#"
        package.path = package.path .. ';./?.lua'
    "#).exec()?;

    let main_script = fs::read_to_string("extensions/interface/db_interface.lua")
        .expect("Não foi possivel carregar o script principal");

    lua.load(&main_script).exec()?;

    Ok(())
}