tweet

This library is for deserializing data from the Twitter API.

I ended up creating this since I couldn't find a way to deserialize data while using `twitter-stream`, and `twitter-stream-message` seems to be an abandoned project that does not work currently.

If there is anything missing feel free to create an issue. I tried to add every field regardless of potential use, but there were some I left out; mainly enrichment stuff, or things with no concrete docs I could find.

## Example

```rust
use tweet::Tweet;

Tweet::from_str(&json)
```
