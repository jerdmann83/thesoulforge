#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <zmq.h>

void bail(const char* msg) {
    fprintf(stderr, "%s\n", msg);
    exit(-1);
}

void help() {
    bail("expect: [req|rep|pub|sub]");
}

char* zstr(zmq_msg_t* msg) {
    const char* zd = (const char*)zmq_msg_data(msg);
    int zlen = zmq_msg_size(msg);
    char* out = strndup(zd, zlen);
    return out;
}

char* zbuf(zmq_msg_t* msg) {
    return (char*)zmq_msg_data(msg);
}

void zsend_seq(void* sock, const char* tag, int seq) {
    zmq_msg_t msg;
    const int len = 16;
    zmq_msg_init_size(&msg, len);
    char* buf = zbuf(&msg);
    snprintf(buf, len, "%s%03i", tag, seq);
    printf("sending %s... ", buf);
    fflush(stdout);
    zmq_msg_send(&msg, sock, 0);
    printf("sent!\n");
    zmq_msg_close(&msg);
}

void zrecv(void* sock, zmq_msg_t* msg) {
    zmq_msg_init(msg);
    printf("rcving... ");
    fflush(stdout);
    zmq_msg_recv(msg, sock, 0);
    char* buf = zbuf(msg);
    printf("rcv %s\n", buf);
    zmq_msg_close(msg);
}

struct znode {
    void* context;
    void* socks[16];
    size_t sockidx;
};

void znode_init(znode* zn) {
    zn->context = zmq_ctx_new();
    for (size_t i=0; i<sizeof(zn->socks); ++i) zn->socks[i] = NULL;
    zn->sockidx = 0;
}

void znode_add_socket(znode* zn, void* sock) {
    zn->socks[zn->sockidx] = sock;
    zn->sockidx++;
}

int rep() {
    void* context = zmq_ctx_new();
    void* sock = zmq_socket(context, ZMQ_REP);
    int rc = zmq_bind(sock, "tcp://*:5555");
    assert(rc == 0);

    int seq = 0;
    while (1) {
        zmq_msg_t msg;
        zrecv(sock, &msg);

        sleep(1);
        zsend_seq(sock, "rep", ++seq);
    }

    zmq_close(sock);
    zmq_ctx_destroy(context);
    return 0;
}

int req() {
    void* context = zmq_ctx_new();
    void* sock = zmq_socket(context, ZMQ_REQ);
    int rc = zmq_connect(sock, "tcp://127.0.0.1:5555");
    assert(rc == 0);

    int seq = 0;
    while (1) {
        zsend_seq(sock, "req", ++seq);
        sleep(1);
        zmq_msg_t msg;
        zrecv(sock, &msg);
    }

    zmq_close(sock);
    zmq_ctx_destroy(context);
    return 0;
}

int pub() {
    void* context = zmq_ctx_new();
    void* sock = zmq_socket(context, ZMQ_PUB);
    int rc = zmq_bind(sock, "tcp://127.0.0.1:5555");
    assert(rc == 0);
    rc = zmq_bind(sock, "ipc://zpub");
    assert(rc == 0);

    int seq = 0;
    while (1) {
        zsend_seq(sock, "pub", ++seq);
        sleep(1);
    }

    zmq_close(sock);
    zmq_ctx_destroy(context);
    return 0;
}

void zsubscribe(void* sock, const char* filter) {
    int rc = zmq_setsockopt(sock, 
            ZMQ_SUBSCRIBE,
            filter, 
            strlen(filter));
    assert(rc == 0);
}

int sub() {
    void* context = zmq_ctx_new();
    void* sock = zmq_socket(context, ZMQ_SUB);
    int rc = zmq_connect(sock, "tcp://127.0.0.1:5555");
    assert(rc == 0);

    zsubscribe(sock, "");

    while (1) {
        zmq_msg_t msg;
        zrecv(sock, &msg);
    }

    zmq_close(sock);
    zmq_ctx_destroy(context);
    return 0;
}

int main(int argc, char** argv) {
    if (argc < 2) {
        help();
    }

    if (strcmp(argv[1], "req") == 0) {
        return req();
    } else if (strcmp(argv[1], "rep") == 0) {
        return rep();
    } else if (strcmp(argv[1], "pub") == 0) {
        return pub();
    } else if (strcmp(argv[1], "sub") == 0) {
        return sub();
    } else {
        help();
    }
}
