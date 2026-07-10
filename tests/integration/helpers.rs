use std::io::Write;

/// Builds a minimal in-memory ZIP that passes SquareCloud upload validation.
///
/// Contains a `squarecloud.app` config and a no-op `index.js` so the platform
/// accepts it without errors. The display name is fixed so tests can assert on
/// it.
pub fn dummy_zip() -> Vec<u8> {
    let buf = std::io::Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);
    let opts = zip::write::SimpleFileOptions::default();

    zip.start_file("squarecloud.app", opts)
        .expect("zip: failed to start squarecloud.app entry");
    zip.write_all(
        b"DISPLAY_NAME=squarecloud-rs-test\n\
          MAIN=index.js\n\
          MEMORY=512\n\
          VERSION=recommended\n\
          SUBDOMAIN=squarecloud-rs-test\n",
    )
    .expect("zip: failed to write squarecloud.app content");

    zip.start_file("index.js", opts)
        .expect("zip: failed to start index.js entry");
    zip.write_all(
        b"const http = require('http');\n\
          http.createServer((_, res) => res.end('ok')).listen(80);\n\
          setInterval(() => console.log('ping'), 1000);\n",
    )
    .expect("zip: failed to write index.js content");

    zip.start_file("package.json", opts)
        .expect("zip: failed to start package.json entry");
    zip.write_all(
        b"{\"name\":\"squarecloud-rs-test\",\"version\":\"1.0.0\"}\n",
    )
    .expect("zip: failed to write package.json content");

    zip.finish()
        .expect("zip: failed to finalize archive")
        .into_inner()
}
