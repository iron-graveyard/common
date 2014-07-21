#![crate_name = "core"]
#![feature(globs)]

extern crate realiron = "iron";
extern crate realurlencoded = "urlencoded";
extern crate reallogger = "logger";
extern crate realrouter = "router";
extern crate realmount = "mount";
extern crate realpersistent = "persistent";
extern crate realsession = "session";
extern crate realcookie = "cookie";
extern crate realbodyparser = "bodyparser";

pub mod iron { pub use realiron::*; }

pub mod urlencoded { pub use realurlencoded::*; }

pub mod logger { pub use reallogger::*; }

pub mod router { pub use realrouter::*; }

pub mod mount { pub use realmount::*; }

pub mod persistent { pub use realpersistent::*; }

pub mod session { pub use realsession::*; }

pub mod cookie { pub use realcookie::*; }

pub mod bodyparser { pub use realbodyparser::*; }

