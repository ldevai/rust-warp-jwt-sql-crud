#!/bin/sh
systemfd --no-pid -s http::0.0.0.0:3535 -- cargo watch -x run
