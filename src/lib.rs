mod wav;
use neon::prelude::*;

/// Argument 0: path string
fn get_samples_from_file(mut cx: FunctionContext) -> JsResult<JsString> {
    // let path = "PinkPanther60.wav";
    let path: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let samples_per_pixel = 250;
    let width = 800;

    // TODO: Handle unwraps
    let summary = wav::extract_samples(&path, samples_per_pixel, &width).unwrap();
    let string = wav::serialize_to_json(&summary).unwrap();
    Ok(cx.string(string))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_samples_from_file", get_samples_from_file)?;
    Ok(())
}
