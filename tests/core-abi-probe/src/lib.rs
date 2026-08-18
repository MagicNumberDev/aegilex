wit_bindgen::generate!({
    path: "wit",
    world: "probe-world",
});

use aegilex::core_abi_probe::probe::{self, EchoInput};
use exports::aegilex::core_abi_probe::callbacks;

use std::cell::RefCell;

thread_local! {
    static SINK: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

struct Probe;

impl Guest for Probe {
    fn entry() -> String {
        let input = EchoInput {
            message: "hello".to_owned(),
            count: 7,
        };
        let echoed = probe::echo(&input);
        let maybe = probe::maybe(Some(echoed.len() as u32)).unwrap_or_default();
        probe::log(&format!("entry: {echoed} maybe={maybe}"));
        format!("{echoed}/{maybe}")
    }
}

impl callbacks::Guest for Probe {
    type Sink = ProbeSink;

    fn on_tick() -> Result<(), String> {
        Ok(())
    }
}

struct ProbeSink;

impl callbacks::GuestSink for ProbeSink {
    fn push(&self, value: String) {
        SINK.with(|sink| sink.borrow_mut().push(value));
    }

    fn drain(&self) -> Vec<String> {
        SINK.with(|sink| std::mem::take(&mut *sink.borrow_mut()))
    }
}

export!(Probe);
