typedef struct 
{
    int i;
    const int const_i;
    
    char c;
    const char const_c;

    double d;
    float f;

    char array[4];
    
    void* buffer;
} t_session;

typedef struct 
{
    void* data;
} t_buffer;

t_session* start();
t_buffer* session_buffer(t_session *session);
char* buffer_data(t_buffer *buffer);
void stop(t_session *session);

char* pong(char * ping);
char* pong_char(char c);