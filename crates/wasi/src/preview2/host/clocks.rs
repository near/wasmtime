#![allow(unused_variables)]

use crate::preview2::bindings::{
    clocks::monotonic_clock::{self, Duration as WasiDuration, Instant},
    clocks::timezone::{self, TimezoneDisplay},
    clocks::wall_clock::{self, Datetime},
};
use crate::preview2::poll::{subscribe, Subscribe};
use crate::preview2::{Pollable, WasiView};
use cap_std::time::SystemTime;
use std::time::Duration;
use wasmtime::component::Resource;

impl TryFrom<SystemTime> for Datetime {
    type Error = anyhow::Error;

    fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
        let duration =
            time.duration_since(SystemTime::from_std(std::time::SystemTime::UNIX_EPOCH))?;

        Ok(Datetime {
            seconds: duration.as_secs(),
            nanoseconds: duration.subsec_nanos(),
        })
    }
}

impl<T: WasiView> wall_clock::Host for T {
    fn now(&mut self) -> anyhow::Result<Datetime> {
        let now = self.ctx().wall_clock.now();
        Ok(Datetime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        })
    }

    fn resolution(&mut self) -> anyhow::Result<Datetime> {
        let res = self.ctx().wall_clock.resolution();
        Ok(Datetime {
            seconds: res.as_secs(),
            nanoseconds: res.subsec_nanos(),
        })
    }
}

fn subscribe_to_duration(
    table: &mut crate::preview2::Table,
    duration: tokio::time::Duration,
) -> anyhow::Result<Resource<Pollable>> {
    let sleep = if duration.is_zero() {
        table.push(Deadline::Past)?
    } else if let Some(deadline) = tokio::time::Instant::now().checked_add(duration) {
        // NB: this resource created here is not actually exposed to wasm, it's
        // only an internal implementation detail used to match the signature
        // expected by `subscribe`.
        table.push(Deadline::Instant(deadline))?
    } else {
        // If the user specifies a time so far in the future we can't
        // represent it, wait forever rather than trap.
        table.push(Deadline::Never)?
    };
    subscribe(table, sleep)
}

impl<T: WasiView> monotonic_clock::Host for T {
    fn now(&mut self) -> anyhow::Result<Instant> {
        Ok(self.ctx().monotonic_clock.now())
    }

    fn resolution(&mut self) -> anyhow::Result<Instant> {
        Ok(self.ctx().monotonic_clock.resolution())
    }

    fn subscribe_instant(&mut self, when: Instant) -> anyhow::Result<Resource<Pollable>> {
        let clock_now = self.ctx().monotonic_clock.now();
        let duration = if when > clock_now {
            Duration::from_nanos(when - clock_now)
        } else {
            Duration::from_nanos(0)
        };
        subscribe_to_duration(&mut self.table_mut(), duration)
    }

    fn subscribe_duration(&mut self, duration: WasiDuration) -> anyhow::Result<Resource<Pollable>> {
        subscribe_to_duration(&mut self.table_mut(), Duration::from_nanos(duration))
    }
}

enum Deadline {
    Past,
    Instant(tokio::time::Instant),
    Never,
}

#[async_trait::async_trait]
impl Subscribe for Deadline {
    async fn ready(&mut self) {
        match self {
            Deadline::Past => {}
            Deadline::Instant(instant) => tokio::time::sleep_until(*instant).await,
            Deadline::Never => std::future::pending().await,
        }
    }
}

impl<T: WasiView> timezone::Host for T {
    fn display(&mut self, when: Datetime) -> anyhow::Result<TimezoneDisplay> {
        todo!("timezone display is not implemented")
    }

    fn utc_offset(&mut self, when: Datetime) -> anyhow::Result<i32> {
        todo!("timezone utc_offset is not implemented")
    }
}
