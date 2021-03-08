//// Copyright © 2016, Canal TP and/or its affiliates. All rights reserved.
////
//// This file is part of Navitia,
////     the software to build cool stuff with public transport.
////
//// Hope you'll enjoy and contribute to this project,
////     powered by Canal TP (www.canaltp.fr).
//// Help us simplify mobility and open public transport:
////     a non ending quest to the responsive locomotion way of traveling!
////
//// LICENCE: This program is free software; you can redistribute it
//// and/or modify it under the terms of the GNU Affero General Public
//// License as published by the Free Software Foundation, either
//// version 3 of the License, or (at your option) any later version.
////
//// This program is distributed in the hope that it will be useful, but
//// WITHOUT ANY WARRANTY; without even the implied warranty of
//// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//// Affero General Public License for more details.
////
//// You should have received a copy of the GNU Affero General Public
//// License along with this program. If not, see
//// <http://www.gnu.org/licenses/>.
////
//// Stay tuned using
//// twitter @navitia
//// IRC #navitia on freenode
//// https://groups.google.com/d/forum/navitia
//// www.navitia.io
//
//// use slog::{debug, info, trace};
//use slog::{self, o, slog_o, Drain, Never};
//use slog_scope::{debug, info, trace};
//use std::env;
//
//fn main() -> Result<(), String> {
//    let _guard = configure_logger(
//        slog_term::CompactFormat::new(slog_term::PlainDecorator::new(std::io::stderr()))
//            .build()
//            .fuse(),
//    );
//
//    trace!("trace bragi");
//    debug!("debug bragi");
//    info!("info bragi");
//    debug!("running bragi webserver");
//    // bragi::server::runserver()
//    Ok(())
//}
//
//fn configure_logger<T>(drain: T) -> (slog_scope::GlobalLoggerGuard, ())
//where
//    T: Drain<Ok = (), Err = Never> + Send + 'static,
//{
//    //by default we log for info
//    let builder = slog_envlogger::LogBuilder::new(drain).filter(None, slog::FilterLevel::Trace);
//    let builder = if let Ok(s) = env::var("RUST_LOG") {
//        builder.parse(&s)
//    } else {
//        builder
//    };
//    let drain = slog_async::Async::new(builder.build())
//        .chan_size(256)
//        .build();
//
//    let log = slog::Logger::root(drain.fuse(), slog_o!());
//    let scope_guard = slog_scope::set_global_logger(log);
//    slog_stdlog::init_with_level(log::Level::Trace).unwrap();
//    (scope_guard, ())
//} // let _guard = mimir::logger_init();

use slog::Drain;

fn main() {
    let drain = slog_async::Async::default(slog_envlogger::new(
        slog_term::CompactFormat::new(slog_term::TermDecorator::new().stderr().build())
            .build()
            .fuse(),
    ));

    let root_logger = slog::Logger::root(
        drain.fuse(),
        slog::slog_o!("build" => "8jdkj2df", "version" => "0.1.5"),
    );

    // slog_stdlog::init_with_level(log::Level::Trace).unwrap();
    slog_stdlog::init().unwrap();

    slog_scope::scope(&root_logger, || {
        slog::slog_error!(root_logger, "slog error");
        slog_scope::error!("log error");
        slog::slog_info!(root_logger, "slog info");
        slog_scope::info!("log info");
        slog::slog_trace!(root_logger, "slog trace");
        slog_scope::trace!("log trace");
    });
}
