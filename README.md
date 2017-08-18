# serde_with

Based on the idea stated in this serde issue:
https://github.com/serde-rs/serde/issues/553

## TODO

* [ ] Deserialize an empty string as `None`
* Chrono
    * [ ] Like `ts_seconds` but for milli/nano-seconds
    * [ ] Flexible number of digits for subsecond precision
* [ ] Stringify `true/false`, `True/False` (for Python)
* [ ] Derserialize a String which is itself Json, Yaml, etc.