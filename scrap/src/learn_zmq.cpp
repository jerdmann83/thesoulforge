#include "learn_zmq.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <zmq.h>

char* make_endpoint(const char* addr, int port) {
    char* ep = (char*)malloc(64);
    snprintf(ep, 64, "tcp://%s:%i", addr, port);
    printf("ep: %s\n", ep);
    return ep;
}

int server(int port) {
    void* context = zmq_ctx_new();
    void* responder = zmq_socket(context, ZMQ_REP);
    char* ep = make_endpoint("*", port);
    assert(zmq_bind(responder, ep) == 0);

    while (1) {
        char buf[128];
        int len = zmq_recv(responder, buf, sizeof(buf), ZMQ_DONTWAIT);
        if (len < 0) {
            sleep(1);
            continue;
        }
        buf[len] = 0;
        int pos = 0;
        while (buf[pos] < '0' || buf[pos] > '9') ++pos;
        int no = atoi(buf + pos);
        printf("recv: %s\n", buf);
        snprintf(buf, sizeof(buf), "ack%06i", no);
        zmq_send(responder, buf, strlen(buf), 0);
    }

    zmq_close(ep);
    free(ep);
    zmq_ctx_destroy(context);
    return 0;
}

int client(int port) {
    void* context = zmq_ctx_new();
    void* requester = zmq_socket(context, ZMQ_REQ);
    char* ep = make_endpoint("localhost", port);
    char buf[128];
    printf("connecting...\n");
    zmq_connect(requester, ep);
    printf("connected!\n");

    int no = 0;
    while (1) {
        snprintf(buf, sizeof(buf), "msg%06i", ++no);
        zmq_send(requester, buf, strlen(buf), 0);
        sleep(1);
        int len = zmq_recv(requester, buf, sizeof(buf), ZMQ_DONTWAIT);
        if (len < 0) {
            continue;
        }
        buf[len] = 0;
        printf("recv: %s\n", buf);
    }
    zmq_close(ep);
    free(ep);
    zmq_ctx_destroy(context);
    return 0;
}

