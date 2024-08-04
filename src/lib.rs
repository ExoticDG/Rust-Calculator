slint::include_modules!();

mod calculator;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let ui = AppWindow::new().unwrap();
    ui.on_calc({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let n1 = ui.get_num1ui();
            let n2 = ui.get_num2ui();
            let s = ui.get_symui();
            let ans = match calculator::calculate(s.to_string(), n1.to_string(), n2.to_string()) {
                Ok(x) => x,
                Err(e) => e,
            };
            ui.set_result(ans.into());
        }
    });
    ui.run().unwrap();
}