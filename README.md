## Config

```
# Specifies the outputs. `stdout`, `email` or a file with optional date formatting (defaults to `stdout`)
outputs = ["stdout", "/tmp/alerty/output_%Y%m%d%H%M%S.html"]
# (Optional) specifies a template to use for formatting. If not set a prebuilt one will be used
output_template_path = "tmp/test_template.txt"
# (Optional) Specifies the path for the alerty database (default in example)
database_path = "~/.config/alerty/database.json"

# Specifies the smtp path to used. Only required if `email` is one of the outputs
[smtp]
relay = "smtp.gmail.com"
username = "me@gmail.com"
password = "..."
to = "<me> me@gmail.com"
from = "<me> me@gmail.com"
subject = "Alerty: new updates for %d.%m.%Y"

# Specify the sources to use
[[instagram]]
username = "lowroar"
[[bandwear]]
shop_name = "low-roar-shop"
```

## Templates

Uses [`minijinja`](https://docs.rs/minijinja/latest/minijinja/) to render.

#### Variables

`results`: A mapping of:

```
{
    "source": {
        "source-id": [
            {
                "id": "post-id",
                "title": "post-title" | None,
                "text": "post-text" | None,
                "link": "post-link" | None,
                "thumbnail": "post-thumbnail" | None
            },
            ...
        ]
    }
}
```

`errors`: A mapping of:

```
[
    ("source", "source-id", "error message"),
    ...
]
```

#### Custom functions

`current_date("%format")`: format the current date.

#### Custom filters

`split("separator")`: splits the string by the the given separator. e.g. `data.text|split("\n")` to split the text by lines.

## Sources

#### Instagram

Scrapes an instagram `username` for new posts.

NOTE: the thumbnail urls returned are protected by CORS, so don't render in html output `<img>` tags.

```
[[instagram]]
username = "lowroar"
```

#### Bandwear

Scrapes a [Bandwear](https://shop.bandwear.com) collection for new products.

```
[[bandwear]]
shop_name = "low-roar-shop"
```
