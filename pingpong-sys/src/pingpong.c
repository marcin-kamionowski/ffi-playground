#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include "pingpong.h"

char* pong(char * ping) {
    char *prefix = "Pong ";
    int size = strlen(prefix) + strlen(ping);
    char *reply = malloc(size + 1);
    sprintf(reply, "%s%s", prefix, ping);
    return reply;
}

char* pong_char(char c) {
    char *prefix = "Pong ";
    int size = strlen(prefix) + 1;
    char *reply = malloc(size + 1);
    sprintf(reply, "%s%d", prefix, c);
    return reply;
}

t_session* start(){
    t_session* s = malloc(sizeof(t_session));
    s->c = 'A';
    t_buffer* b = malloc(sizeof(t_buffer));
    b->data = malloc(50);
    sprintf(b->data, "Buffer data");
    s->buffer = b;
    return s;
}

t_buffer* session_buffer(t_session *session){
    return session->buffer;
}

char* buffer_data(t_buffer *buffer){
    return buffer->data;
}

void stop(t_session *session){
    t_buffer* b = session_buffer(session);
    free(b->data);
    free(b);
    free(session);
}
