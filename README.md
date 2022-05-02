# sitemap, a Sitemap library for Rust

sitemap is a [sitemap.xml] library for the [Rust](http://www.rust-lang.org/) programming language.

This is a fork of <https://github.com/svmk/rust-sitemap> that uses the [time] crate instead of [chrono].

[sitemap.xml]: https://www.sitemaps.org/protocol.html
[time]: https://lib.rs/crates/time
[chrono]: https://lib.rs/crates/chrono

## Features
* Streaming reading sitemap

## Restrictions
* no other encodings but UTF-8 are supported yet
* validation is not supported

## Reading sitemap documents
`sitemap::reader::SiteMapReader` requires a `Read` instance to read from. When a proper stream-based encoding library is available, it is likely that sitemap will be switched to use whatever character stream structure this library would provide, but currently it is a `Read`.

Using `SiteMapReader` is very straightforward. Just provide a `Read` instance to obtain an iterator over events:

```rust
use sitemap::reader::{SiteMapReader,SiteMapEntity};
use std::fs::File;
fn main() {
    let mut urls = Vec::new();
    let mut sitemaps = Vec::new();
    let mut errors = Vec::new();
    let file = File::open("sitemap.xml").expect("Unable to open file.");
    let parser = SiteMapReader::new(file);
    for entity in parser {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                urls.push(url_entry);
            },
            SiteMapEntity::SiteMap(sitemap_entry) => {
                sitemaps.push(sitemap_entry);
            },
            SiteMapEntity::Err(error) => {
                errors.push(error);
            },
        }
    }
    println!("urls = {:?}",urls);
    println!("sitemaps = {:?}",sitemaps);
    println!("errors = {:?}",errors);
}
```

## Writng sitemap documents
```rust
use sitemap::writer::SiteMapWriter;
use sitemap::structs::UrlEntry;
use std::io::stdout;
fn main() {
    let mut output = stdout();
    let sitemap_writer = SiteMapWriter::new(&mut output);
    let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
    urlwriter.url("http://github.com").expect("Unable to write url");
    urlwriter.url(UrlEntry::builder().loc("http://google.com")).expect("Unable to write url");
    urlwriter.url(UrlEntry::builder().loc("http://yandex.ru").build().unwrap()).expect("Unable to write url");
    urlwriter.end().expect("Unable to write close tags");
}
```

## License

This library is licensed under MIT license.


