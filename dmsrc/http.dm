// HTTP Operations //

#define RUSTG_HTTP_METHOD_GET "get"
#define RUSTG_HTTP_METHOD_PUT "put"
#define RUSTG_HTTP_METHOD_DELETE "delete"
#define RUSTG_HTTP_METHOD_PATCH "patch"
#define RUSTG_HTTP_METHOD_HEAD "head"
#define RUSTG_HTTP_METHOD_POST "post"
#define rustg_http_request_blocking(method, url, body, headers, options) CALL_LIB(RUST_UTILS, "http_request_blocking")(method, url, body, headers, options)
#define rustg_http_request_async(method, url, body, headers, options) CALL_LIB(RUST_UTILS, "http_request_async")(method, url, body, headers, options)
#define rustg_http_check_request(req_id) CALL_LIB(RUST_UTILS, "http_check_request")(req_id)
/proc/rustg_create_async_http_client() return CALL_LIB(RUST_UTILS, "start_http_client")()
/proc/rustg_close_async_http_client() return CALL_LIB(RUST_UTILS, "shutdown_http_client")()
