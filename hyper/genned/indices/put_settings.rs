//Autogenerated
use hyper::Client;
pub fn put(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 10);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_settings");
    url_fmtd
}
pub fn put_index(base: String, index: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_settings");
    url_fmtd
}