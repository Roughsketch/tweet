[![ci-badge][]][ci] [![docs-badge][]][docs] [![crates.io version]][crates.io link]

tweet

This library is for deserializing data from the Twitter API.

I created this since I couldn't find a way to deserialize data while using `twitter-stream`, and `twitter-stream-message` is an abandoned project that does not work currently.

If there is anything missing feel free to create an issue. I tried to add every field regardless of potential use, but there were some I left out; mainly things with no concrete documentation or things that I cannot access like PowerTrack and enterprise endpoint responses.

## Simple example

```rust
use std::str::FromStr;
use tweet::Tweet;

Tweet::from_str(&json)
```

## Usage with twitter-stream
```rust
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use tweet::TwitterResponse;

// Import your keys however you want
let token = Token::new(
    dotenv!("TW_API"), dotenv!("TW_SEC"),
    dotenv!("TW_ACC_KEY"), dotenv!("TW_ACC_SEC"));

let future = TwitterStreamBuilder::filter(token)
    .timeout(None)
    .track(Some("cat, dog, rabbit"))
    .listen().unwrap()
    .flatten_stream()
    .for_each(move |json| {
        //  A twitter stream just sends us raw JSON responses, and those
        //  responses can contain a Tweet or a Limit payload. TwitterResponse
        //  encapsulates deserializing this variable payload. Without it there
        //  is a possibility of trying to deserialize a Limit as a Tweet and
        //  getting a deserialization error.
        let tweet = match TwitterResponse::from_str(&json) {
            //  Return the tweet so we can use it
            Ok(TwitterResponse::Tweet(tweet)) => tweet,
            //  Just print out limit information if we get it and return
            Ok(TwitterResponse::Limit(limit)) => {
                println!("Got a limit: {:#?}", limit);
                return Ok(());
            }
            //  If something goes wrong, print the error and the payload
            Err(why) => {
                println!("Error: {:?}\nPayload: {}", why, json);
                return Ok(());
            }
        };
        
        //  Use tweet however you want
        println!("Tweet URL: {}", tweet.url());

        Ok(())
    })
    .map_err(|e| println!("Error: {}", e));
```

[ci]: https://travis-ci.org/Roughsketch/tweet
[ci-badge]: https://img.shields.io/travis/Roughsketch/tweet.svg?style=flat-square
[crates.io link]: https://crates.io/crates/tweet
[crates.io version]: https://img.shields.io/crates/v/tweet.svg?style=flat-square
[docs]: https://docs.rs/tweet
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
