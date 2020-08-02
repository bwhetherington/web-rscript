mod module_info;
mod utils;

use rscript::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    fn log2(x: &str) -> f64;

    fn log(s: &str);

    fn increment(i: f64) -> f64;

    fn prompt(s: &str) -> String;
}

static mut ENGINE: Option<Engine> = None;

fn evaluate(input: &str, engine: &mut Engine) -> Result<String, String> {
    let lexer = Lexer::new(Some("<input>"), input);
    let mut parser = Parser::new(lexer);
    let expr = parser.parse_expr().map_err(|err| err.to_string())?;
    let value = engine.evaluate(&expr).map_err(|err| err.to_string())?;
    engine
        .value_to_string(&value)
        .map_err(|err| err.to_string())
}

#[wasm_bindgen]
pub fn run(input: &str) -> Option<String> {
    unsafe {
        ENGINE.as_mut().map(|engine| {
            let res = evaluate(input, engine);
            match res {
                Ok(value) => format!("[ok]:Result: {}.", value),
                Err(err) => format!("[err]:Error: {}.", err),
            }
        })
    }
}

fn insert_println(engine: &mut Engine) {
    engine.define_built_in("__print__", |engine, _, args| {
        for arg in args {
            let line = engine.value_to_string(arg)?;
            log(&line);
        }
        Ok(Value::None)
    });
    engine.define_built_in("__unix_time__", |_, _, args| {
        match args {
            [] => {
                // foo
                let res = log2("");
                Ok(Value::Number(res))
            }
            xs => Err(EvalError::arity_mismatch(0, xs.len())),
        }
    });
    engine.define_built_in("__input__", |engine, _, args| match args {
        [arg] => {
            let s = engine.value_to_string(arg)?;
            let input = prompt(&s);
            let output = engine.make_string(input)?;
            Ok(output)
        }
        xs => Err(EvalError::arity_mismatch(1, xs.len())),
    });
}

fn create_engine(std: &str) -> Option<Engine> {
    let mut engine = Engine::new();
    engine.init().unwrap();
    insert_println(&mut engine);

    // Import std
    let module_info = module_info::module_from_json(std).expect("failed to read JSON");
    let module = parse_from_meta(&module_info).expect("failed to parse source");

    // log(&format!("{:#?}", module));

    engine.preload_module(&module);

    // log(&format!("{:#?}", new_engine.root));

    engine
        .load_module(&module, false)
        .expect("failed to load std");

    Some(engine)
}

#[wasm_bindgen]
pub fn init(std: &str) {
    utils::set_panic_hook();

    let engine = create_engine(std);

    unsafe {
        ENGINE = engine;
    }
}
