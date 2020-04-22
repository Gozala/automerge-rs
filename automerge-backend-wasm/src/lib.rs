//#![feature(set_stdio)]

use automerge_backend::{ActorID, AutomergeError, Backend, ChangeRequest, Clock};
use js_sys::{Array, Uint8Array};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate web_sys;
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
          web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn js_to_rust<T: DeserializeOwned>(value: JsValue) -> Result<T, JsValue> {
    value.into_serde().map_err(json_error_to_js)
}

fn rust_to_js<T: Serialize>(value: T) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&value).map_err(json_error_to_js)
}

#[wasm_bindgen]
#[derive(PartialEq, Debug, Clone)]
pub struct State {
    backend: Backend,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl State {
    #[wasm_bindgen(js_name = applyChanges)]
    pub fn apply_changes(&mut self, changes: Array) -> Result<JsValue, JsValue> {
        let ch: Vec<Vec<u8>> = changes
            .iter()
            .map(|c| c.dyn_into::<Uint8Array>().unwrap().to_vec())
            .collect();
        let patch = self
            .backend
            .apply_changes_binary(ch)
            .map_err(automerge_error_to_js)?;
        rust_to_js(&patch)
    }

    #[wasm_bindgen(js_name = loadChanges)]
    pub fn load_changes(&mut self, changes: Array) -> Result<(), JsValue> {
        let ch: Vec<Vec<u8>> = changes
            .iter()
            .map(|c| c.dyn_into::<Uint8Array>().unwrap().to_vec())
            .collect();
        self.backend
            .load_changes_binary(ch)
            .map_err(automerge_error_to_js)
    }

    #[wasm_bindgen(js_name = applyLocalChange)]
    pub fn apply_local_change(&mut self, change: JsValue) -> Result<JsValue, JsValue> {
        let c: ChangeRequest = js_to_rust(change)?;
        let patch = self
            .backend
            .apply_local_change(c)
            .map_err(automerge_error_to_js)?;
        rust_to_js(&patch)
    }

    #[wasm_bindgen(js_name = getPatch)]
    pub fn get_patch(&self) -> Result<JsValue, JsValue> {
        let patch = self.backend.get_patch().map_err(automerge_error_to_js)?;
        rust_to_js(&patch)
    }

    #[wasm_bindgen(js_name = getChanges)]
    pub fn get_changes(&self, clock: JsValue) -> Result<Array, JsValue> {
        let c: Clock = js_to_rust(clock)?;
        let changes = self.backend.get_missing_changes(&c).map_err(automerge_error_to_js)?;
        let result = Array::new();
        for c in changes {
            let bytes : Uint8Array = c.as_slice().into();
            result.push(bytes.as_ref());
        }
        Ok(result)
    }

    #[wasm_bindgen(js_name = getChangesForActor)]
    pub fn get_changes_for_actorid(&self, actorid: JsValue) -> Result<Array, JsValue> {
        let a: ActorID = js_to_rust(actorid)?;
        let changes = self.backend.get_changes_for_actor_id(&a).map_err(automerge_error_to_js)?;
        let result = Array::new();
        for c in changes {
            let bytes : Uint8Array = c.as_slice().into();
            result.push(bytes.as_ref());
        }
        Ok(result)
    }

    #[wasm_bindgen(js_name = getMissingDeps)]
    pub fn get_missing_deps(&self) -> Result<JsValue, JsValue> {
        let clock = self.backend.get_missing_deps();
        rust_to_js(&clock)
    }

    #[wasm_bindgen(js_name = getClock)]
    pub fn get_clock(&self) -> Result<JsValue, JsValue> {
        rust_to_js(&self.backend.clock)
    }

    #[wasm_bindgen(js_name = getUndoStack)]
    pub fn get_undo_stack(&self) -> Result<JsValue, JsValue> {
        rust_to_js(&self.backend.undo_stack)
    }

    #[wasm_bindgen(js_name = getRedoStack)]
    pub fn get_redo_stack(&self) -> Result<JsValue, JsValue> {
        rust_to_js(&self.backend.redo_stack)
    }

    #[wasm_bindgen(js_name = forkAt)]
    pub fn fork_at(&self, _clock: JsValue) -> Result<State, JsValue> {
        let clock: Clock = js_to_rust(_clock)?;
        let changes = self
            .backend
            .history()
            .iter()
            .filter(|change| clock.get(&change.actor_id) >= change.seq)
            .map(|&c| c.clone())
            .collect();
        let mut fork = State {
            backend: Backend::init(),
        };
        let _patch = fork
            .backend
            .apply_changes(changes)
            .map_err(automerge_error_to_js)?;
        Ok(fork)
    }

    #[wasm_bindgen]
    pub fn new() -> State {
        State {
            backend: Backend::init(),
        }
    }
}

fn automerge_error_to_js(err: AutomergeError) -> JsValue {
    js_sys::Error::new(&std::format!("Automerge error: {}", err)).into()
}

fn json_error_to_js(err: serde_json::Error) -> JsValue {
    js_sys::Error::new(&std::format!("serde_json error: {}", err)).into()
}

/*
struct WasmSTDIO {}

impl std::io::Write for WasmSTDIO {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let string = String::from_utf8_lossy(&buf).into_owned();
        web_sys::console::log_1(&string.into());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn main() {
}
*/
