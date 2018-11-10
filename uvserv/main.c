#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <uv.h>
#include <unistd.h>
#include <sys/prctl.h>

#define DEFAULT_PORT 7000
#define DEFAULT_BACKLOG 128

uv_loop_t *loop;
struct sockaddr_in addr;

typedef struct {
    uv_write_t req;
    uv_buf_t buf;
} write_req_t;

typedef struct timer_ctx_s {
    int count;
} timer_ctx_t;

void free_write_req(uv_write_t *req) {
    write_req_t *wr = (write_req_t*) req;
    free(wr->buf.base);
    free(wr);
}

void alloc_buffer(uv_handle_t *handle, size_t suggested_size, uv_buf_t *buf) {
    printf("alloc_buffer: %lu bytes\n", suggested_size);
    buf->base = (char*) malloc(suggested_size);
    buf->len = suggested_size;
}

void on_close(uv_handle_t* handle) {
    printf("on_close: %p\n", handle);
    free(handle);
}

void echo_write(uv_write_t *req, int status) {
    if (status) {
        fprintf(stderr, "Write error %s\n", uv_strerror(status));
    }
    free_write_req(req);
}

void echo_read(uv_stream_t *client, ssize_t nread, const uv_buf_t *buf) {
    printf("echo_read: nread=%lu client=%p\n", nread, client);
    if (nread > 0) {
        write_req_t *req = (write_req_t*) malloc(sizeof(write_req_t));
        req->buf = uv_buf_init(buf->base, nread);
        uv_write((uv_write_t*) req, client, &req->buf, 1, echo_write);
        return;
    }
    if (nread < 0) {
        if (nread != UV_EOF)
            fprintf(stderr, "Read error %s\n", uv_err_name(nread));
        uv_close((uv_handle_t*) client, on_close);
    }

    free(buf->base);
}

void on_new_connection(uv_stream_t *server, int status) {
    if (status < 0) {
        fprintf(stderr, "New connection error %s\n", uv_strerror(status));
        // error!
        return;
    }

    uv_tcp_t *client = (uv_tcp_t*) malloc(sizeof(uv_tcp_t));
    uv_tcp_init(loop, client);
    if (uv_accept(server, (uv_stream_t*) client) == 0) {
        printf("uv_read_start: client=%p\n", client);
        uv_read_start((uv_stream_t*) client, alloc_buffer, echo_read);
    }
    else {
        uv_close((uv_handle_t*) client, on_close);
    }
}

void tick_timer(uv_timer_t* handle) {
    timer_ctx_t* ctx = (timer_ctx_t*)handle->data;
    //printf("=== tick %i\n", ++ctx->count);
}

typedef struct wake_ctx_s {
    uint64_t call_count;
    uint64_t wake_count;
} wake_ctx_t;

void on_wake(uv_async_t* handle) {
    wake_ctx_t* ctx = (wake_ctx_t*)handle->data;
    if (++ctx->call_count % 5000 == 0) {
        printf("on_wake: wake=%li call=%li\n", ctx->wake_count, ctx->call_count);
    }
}

void* run_uv_loop(void* _) {
    prctl(PR_SET_NAME, "uv");
    uv_run(loop, UV_RUN_DEFAULT);
}

int main() {
    loop = uv_default_loop();

    uv_tcp_t server;
    uv_tcp_init(loop, &server);

    timer_ctx_t timer_ctx = {0};

    uv_timer_t timer;
    uv_timer_init(loop, &timer);
    timer.data = &timer_ctx;
    uv_timer_start(&timer, tick_timer, 1000, 1000);

    uv_ip4_addr("0.0.0.0", DEFAULT_PORT, &addr);

    uv_tcp_bind(&server, (const struct sockaddr*)&addr, 0);
    int r = uv_listen((uv_stream_t*) &server, DEFAULT_BACKLOG, on_new_connection);
    if (r) {
        fprintf(stderr, "listen error %s\n", uv_strerror(r));
        return 1;
    }

    pthread_t t1;
    pthread_create(&t1, NULL, run_uv_loop, NULL);

    wake_ctx_t wake_ctx;
    memset(&wake_ctx, 0, sizeof(wake_ctx_t));
    uv_async_t async;
    async.data = &wake_ctx;
    uv_async_init(loop, &async, on_wake);

    // snippet to demonstrate libuv's coalescing of async wake requests
    while (1)
    {
        usleep(100);
        ++wake_ctx.wake_count;
        uv_async_send(&async);
    }

    pthread_join(t1, NULL);
}
