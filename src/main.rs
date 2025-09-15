use mlua::{Lua, Result as LuaResult};
use memory_database::MiniDb;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let db = Rc::new(RefCell::new(MiniDb::new()));

    let db_clone = db.clone();
    let add_fn = lua.create_function(move |_, entry: String| {
        db_clone.borrow_mut().add(entry);
        Ok(())
    })?;
    lua.globals().set("ADD", add_fn)?;

    let db_clone = db.clone();
    let get_fn = lua.create_function(move |_, key: String| {
        Ok(db_clone.borrow().get(&key))
    })?;
    lua.globals().set("GET", get_fn)?;

    let main_script = fs::read_to_string("src/extensions/interface/db_interface.lua")
        .expect("Não foi possível carregar o script principal");

    lua.load(&main_script).exec()?;

    Ok(())
}
