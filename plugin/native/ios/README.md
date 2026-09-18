# iOS reference backend

Drop `{{crate_name | pascal_case}}BackendImpl.swift` into your app target and
register the dispatcher inside `AppDelegate` / `@main App`:

```swift
IstmoRuntime.shared.registerHandler(
    {{crate_name | pascal_case}}Dispatcher.PLUGIN_ID,
    {{crate_name | pascal_case}}Dispatcher(
        backend: {{crate_name | pascal_case}}BackendImpl(),
        codecs:  {{crate_name | pascal_case}}CodecsImpl()
    )
)
```
