#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
    #define UNICODE
    #define _UNICODE
    #define WIN32_LEAN_AND_MEAN
    
    #include <windows.h>
    #include <winsock2.h>
    #include <ws2tcpip.h>
    #pragma comment(lib, "ws2_32.lib")
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <unistd.h>
    #include <errno.h>
#endif

void aura_print_int(long long val) {
    printf("%lld\n", val);
}

void aura_print_str(const char* val) {
    printf("%s\n", val);
}

long long aura_str_contains(const char* buffer, const char* pattern) {
    if (strstr(buffer, pattern) != NULL) return 1;
    return 0;
}

char* aura_str_find(char* str, const char* pattern) {
    return strstr(str, pattern);
}

long long aura_net_setup(long long port) {
#ifdef _WIN32
    WSADATA wsa;
    WSAStartup(MAKEWORD(2, 2), &wsa);
#endif

    long long sock = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons((unsigned short)port);

    bind(sock, (struct sockaddr*)&addr, sizeof(addr));
    listen(sock, 5);
    printf("Aura Runtime: Listening on port %lld\n", port);
    return sock;
}

void aura_close_socket(long long sock) {
#ifdef _WIN32
    closesocket(sock);
#else
    close(sock);
#endif
}

char* aura_read_file(const char* path) {
    FILE* f = fopen(path, "rb");
    if (!f) return "File not found";
    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET);

    char* string = malloc(fsize + 1);
    fread(string, fsize, 1, f);
    fclose(f);
    string[fsize] = 0;
    return string;
}

char* aura_str_replace(const char* orig, const char* rep, const char* with) {
    char* result; 
    char* ins;    
    char* tmp;    
    int len_rep;  
    int len_with; 
    int len_front; 
    int count;    

    if (!orig || !rep) return NULL;
    len_rep = strlen(rep);
    if (len_rep == 0) return NULL; 
    if (!with) with = "";
    len_with = strlen(with);

    ins = (char*)orig;
    for (count = 0; (tmp = strstr(ins, rep)); ++count) {
        ins = tmp + len_rep;
    }

    tmp = result = malloc(strlen(orig) + (len_with - len_rep) * count + 1);

    if (!result) return NULL;

    while (count--) {
        ins = strstr(orig, rep);
        len_front = ins - orig;
        tmp = strncpy(tmp, orig, len_front) + len_front;
        tmp = strcpy(tmp, with) + len_with;
        orig += len_front + len_rep; 
    }
    strcpy(tmp, orig);
    return result;
}

char* aura_int_to_str(long long n) {
    char* s = malloc(30);
    sprintf(s, "%lld", n);
    return s;
}

char* aura_render_field(char* tpl, char* key, long long value) {
    // Threshold heuristic to distinguish pointer vs small integer
    // 0x10000 is a safe bet for modern OS minimum memory address
    if (value > 0x10000 || value < -0x10000) {
        return aura_str_replace(tpl, key, (char*)value);
    } else {
        char* str_val = aura_int_to_str((int)value);
        char* res = aura_str_replace(tpl, key, str_val);
        free(str_val);
        return res;
    }
}
