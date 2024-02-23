use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("zkasm");

    // Benchmarking
    settings.add_bool(
        "instrument_inst",
        "Instrument `Inst` to identify hot instructions.",
        "Inserts calls to `zkevm-proverjs` helpers that trace `Inst`s \
        executed at runtime",
        false,
    );

    TargetIsa::new("zkasm", settings.build())
}
