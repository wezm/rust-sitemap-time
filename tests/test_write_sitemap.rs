use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use sitemap::writer::SiteMapWriter;
use time::macros::datetime;

static CONTENT: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">
  <url>
    \
     <loc>http://www.example.com/index.html</loc>
    \
     <lastmod>2016-07-08T09:10:11Z</lastmod>
    <changefreq>daily</changefreq>
    \
     <priority>0.2</priority>
  </url>
  <url>
    <loc>http://www.example.com/other.html</loc>
    \
     <lastmod>2016-07-18T09:10:11Z</lastmod>
    <changefreq>monthly</changefreq>
    \
     <priority>0.1</priority>
  </url>
</urlset>
<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">
  <sitemap>
    \
     <loc>http://www.example.com/other_sitemap.xml</loc>
    \
     <lastmod>2016-07-18T09:10:11Z</lastmod>
  </sitemap>
</sitemapindex>";

#[test]
fn test_write_sitemap() {
    let mut output = Vec::<u8>::new();
    {
        let sitemap_writer = SiteMapWriter::new(&mut output);
        let mut urlwriter = sitemap_writer.start_urlset().expect("Can't write the file");
        let date = datetime!(2016-07-08 9:10:11 +0:00);
        let url_entry = UrlEntry::builder()
            .loc("http://www.example.com/index.html")
            .changefreq(ChangeFreq::Daily)
            .priority(0.2)
            .lastmod(date)
            .build()
            .expect("valid");
        urlwriter.url(url_entry).expect("Can't write the file");
        let date1 = datetime!(2016-7-18 9:10:11 +0:00);
        let url_entry = UrlEntry::builder()
            .loc("http://www.example.com/other.html")
            .changefreq(ChangeFreq::Monthly)
            .priority(0.1)
            .lastmod(date1)
            .build()
            .expect("valid");
        urlwriter.url(url_entry).expect("Can't write the file");
        let sitemap_writer = urlwriter.end().expect("close the urlset block");

        let mut sitemap_index_writer = sitemap_writer
            .start_sitemapindex()
            .expect("start sitemap index tag");
        let sitemap_entry = SiteMapEntry::builder()
            .loc("http://www.example.com/other_sitemap.xml")
            .lastmod(date1)
            .build()
            .expect("valid");
        sitemap_index_writer
            .sitemap(sitemap_entry)
            .expect("Can't write the file");
        sitemap_index_writer.end().expect("close sitemap block");
    }
    assert_eq!(
        std::str::from_utf8(&output),
        std::str::from_utf8(CONTENT.as_bytes())
    );
}

#[test]
fn test_validation() {
    assert!(UrlEntry::builder().build().is_err());
    assert!(UrlEntry::builder().priority(2.0).build().is_err());
    assert!(UrlEntry::builder().priority(-1.0).build().is_err());
    assert!(SiteMapEntry::builder().build().is_err());
}
