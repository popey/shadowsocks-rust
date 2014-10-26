// The MIT License (MIT)

// Copyright (c) 2014 Y. T. CHUNG

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::sync::{Arc, Mutex};

use relay::loadbalancing::server::LoadBalancer;
use config::{Config, MultipleServer, SingleServer, ServerConfig, ServerConfigVariant};

#[deriving(Clone)]
pub struct RoundRobin {
    server: ServerConfigVariant,
    index: Arc<Mutex<uint>>,
}

impl RoundRobin {
    pub fn new(config: Config) -> RoundRobin {
        RoundRobin {
            server: config.server.expect("server should not be None"),
            index: Arc::new(Mutex::new(0u)),
        }
    }
}

impl LoadBalancer for RoundRobin {
    fn pick_server<'a>(&'a mut self) -> &'a ServerConfig {
        match self.server {
            SingleServer(ref s) => s,
            MultipleServer(ref slist) => {
                let mtx = self.index.deref();
                let mut idx = mtx.lock();
                let ref s = slist[*idx];
                *idx = (*idx + 1) % slist.len();
                s
            }
        }
    }
}