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

    zip.start_file("squarecloud.app", opts).unwrap();
    zip.write_all(
        b"DISPLAY_NAME=squarecloud-rs-test\n\
          MAIN=index.js\n\
          MEMORY=512\n\
          VERSION=recommended\n\
          SUBDOMAIN=squarecloud-rs-test\n",
    )
    .unwrap();

    zip.start_file("index.js", opts).unwrap();
    zip.write_all(
        b"const http = require('http');\n\
          http.createServer((_, res) => res.end('ok')).listen(80);\n",
    )
    .unwrap();

    zip.start_file("package.json", opts).unwrap();
    zip.write_all(
        b"{\"name\":\"squarecloud-rs-test\",\"version\":\"1.0.0\"}\n",
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}
