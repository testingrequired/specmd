use cut::prelude::*;

const EXAMPLE: &'static str = "---
test: example
---

```shell
echo 'Hello World'
```

```stdout
```

```stderr
```

```exit
```
";

fn main() -> CutResult<()> {
    let parsed_cut_file = parse(EXAMPLE)?;

    dbg!(parsed_cut_file);

    Ok(())
}
