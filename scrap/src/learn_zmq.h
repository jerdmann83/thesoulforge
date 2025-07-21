#pragma once

char* make_endpoint(const char* addr, int port);
int server(int port);
int client(int port);
