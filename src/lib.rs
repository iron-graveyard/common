#![crate_name = "common"]
#![feature(globs)]

extern crate "iron" as realiron;
extern crate "urlencoded" as realurlencoded;
extern crate "logger" as reallogger;
extern crate "router" as realrouter;
extern crate "mount" as realmount;
extern crate "persistent" as realpersistent;
extern crate "session" as realsession;
extern crate "cookie" as realcookie;
extern crate "bodyparser" as realbodyparser;

pub mod iron { pub use realiron::*; }

pub mod urlencoded { pub use realurlencoded::*; }

pub mod logger { pub use reallogger::*; }

pub mod router { pub use realrouter::*; }

pub mod mount { pub use realmount::*; }

pub mod persistent { pub use realpersistent::*; }

pub mod session { pub use realsession::*; }

pub mod cookie { pub use realcookie::*; }

pub mod bodyparser { pub use realbodyparser::*; }

