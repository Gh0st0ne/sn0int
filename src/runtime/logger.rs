use crate::engine::ctx::State;
use crate::hlua;
use std::sync::Arc;


pub fn info(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("info", hlua::function1(move |msg: String| {
        state.info(msg);
    }))
}

pub fn debug(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("debug", hlua::function1(move |msg: String| {
        state.debug(msg);
    }))
}

pub fn error(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("error", hlua::function1(move |msg: String| {
        state.error(msg);
    }))
}

pub fn status(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("status", hlua::function1(move |msg: String| {
        state.status(msg);
    }))
}
