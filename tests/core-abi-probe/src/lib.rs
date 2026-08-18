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

    fn direct_u32(value: u32) -> u32 {
        value.wrapping_add(1)
    }

    fn direct_i64(value: i64) -> i64 {
        value.wrapping_neg()
    }

    fn direct_f32() -> f32 {
        f32::from_bits(0x4049_0fdb)
    }

    fn direct_f64() -> f64 {
        f64::from_bits(0x4009_21fb_5444_2d18)
    }

    fn empty_list_values() -> Vec<String> {
        Vec::new()
    }

    fn list_values() -> Vec<String> {
        vec!["alpha".to_owned(), "beta".to_owned()]
    }

    fn exercise_list_import() -> u32 {
        probe::list_echo(&[2, 4, 8])
    }
}

impl callbacks::Guest for Probe {
    type Sink = ProbeSink;

    fn on_tick() -> Result<(), String> {
        Ok(())
    }

    fn tick_count() -> u32 {
        7
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
