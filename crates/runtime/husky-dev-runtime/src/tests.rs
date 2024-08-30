use crate::*;
use husky_path_utils::HuskyLangDevPaths;
use husky_standard_devsoul::StandardDevsoul;
use husky_standard_trace_protocol::figure::StandardFigure;

type StandardDevRuntime = DevRuntime<StandardDevsoul>;

#[test]
fn dev_runtime_works() {
    let dev_paths = HuskyLangDevPaths::new();
    let dev_runtime = StandardDevRuntime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
}

#[test]
#[ignore]
#[should_panic]
fn dev_runtimes_should_panic() {
    let dev_paths = HuskyLangDevPaths::new();
    let dev_runtime = StandardDevRuntime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
    std::thread::spawn(move || {
        let dev_runtime = StandardDevRuntime::new(
            &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
            None,
        )
        .unwrap();
    })
    .join();
}
