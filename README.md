a small rust program that shows how to add a new section to a sourcemod plugin (smx) file

this was more of a proof of concept because I was interested in the smx format.
It hardcodes the new section name and the new section's content.

```rust
  let infilename = env::args().nth(1).unwrap_or("smxreader.smx".into());
  let outfilename = env::args().nth(2).unwrap_or("smxreaderplus.smx".into());
  ...
  let my_section_name = ".src.zip\0";
  let my_section_data = b"asdfasdfasdfasdf";
```
