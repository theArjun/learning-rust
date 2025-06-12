use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Create the WebAssembly engine
    let engine = Engine::default();

    // Load the WASM file
    let module = Module::from_file(&engine, "calculator.wasm")?;

    // Create a store
    let mut store = Store::new(&engine, ());

    // Create a linker for handling imports
    let mut linker = Linker::new(&engine);

    // Define the wasm-bindgen required import
    linker.func_wrap("wbg", "__wbindgen_init_externref_table", || {})?;

    // Instantiate the module using the linker
    let instance = linker.instantiate(&mut store, &module)?;

    // Get the exported functions
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let subtract = instance.get_typed_func::<(i32, i32), i32>(&mut store, "subtract")?;
    let multiply = instance.get_typed_func::<(i32, i32), i32>(&mut store, "multiply")?;
    let divide = instance.get_typed_func::<(i32, i32), f64>(&mut store, "divide")?;
    let modulo = instance.get_typed_func::<(i32, i32), f64>(&mut store, "modulo")?;

    // Call the functions and print results
    println!("Add: 10 + 5 = {}", add.call(&mut store, (10, 5))?); // 15
    println!("Subtract: 10 - 5 = {}", subtract.call(&mut store, (10, 5))?); // 5
    println!("Multiply: 10 * 5 = {}", multiply.call(&mut store, (10, 5))?); // 50
    println!("Divide: 10 / 3 = {}", divide.call(&mut store, (10, 3))?); // 3.33...
    println!("Modulo: 10 % 3 = {}", modulo.call(&mut store, (10, 3))?); // 1.0

    Ok(())
}