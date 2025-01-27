// HTTP Operations //

#define RUSTUTILS_HTTP_METHOD_GET "get"
#define RUSTUTILS_HTTP_METHOD_PUT "put"
#define RUSTUTILS_HTTP_METHOD_DELETE "delete"
#define RUSTUTILS_HTTP_METHOD_PATCH "patch"
#define RUSTUTILS_HTTP_METHOD_HEAD "head"
#define RUSTUTILS_HTTP_METHOD_POST "post"
#define rustutils_http_request_blocking(method, url, body, headers, options) CALL_LIB(RUST_UTILS, "http_request_blocking")(method, url, body, headers, options)
#define rustutils_http_request_async(method, url, body, headers, options) CALL_LIB(RUST_UTILS, "http_request_async")(method, url, body, headers, options)
#define rustutils_http_check_request(req_id) CALL_LIB(RUST_UTILS, "http_check_request")(req_id)
/proc/rustutils_create_async_http_client() return CALL_LIB(RUST_UTILS, "start_http_client")()
/proc/rustutils_close_async_http_client() return CALL_LIB(RUST_UTILS, "shutdown_http_client")()
