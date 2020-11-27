use std::error::Error;
use wasmtime::{Engine, Func, Instance, Module, Store};

fn main() -> Result<(), Box<dyn Error>> {
  let engine = Engine::default();
  let store = Store::new(&engine);
  let module = Module::new(&engine, std::include_str!("./hello.wat"))?;

  // First we can create our `log` function, which will simply print out the
  // parameter it receives.
  let log = Func::wrap(&store, |param: i32| {
    println!("log: {}", param);
  });

  // Next we can create our double function which doubles the input it receives.
  let double = Func::wrap(&store, |param: i32| param * 2);

  // When instantiating the module we now need to provide the imports to the
  // instantiation process. This is the second slice argument, where each
  // entry in the slice must line up with the imports in the module.
  let instance = Instance::new(&store, &module, &[log.into(), double.into()])?;

  let run = instance
    .get_func("run")
    .expect("`run` was not an exported function");

  let run = run.get0::<()>()?;

  Ok(run()?)
}
