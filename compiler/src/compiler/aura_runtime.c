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

void aura_print_int(int val) {
    printf("%d\n", val);
}

void aura_print_str(const char* val) {
    printf("%s\n", val);
}

int aura_str_contains(const char* buffer, const char* pattern) {
    if (strstr(buffer, pattern) != NULL) return 1;
    return 0;
}

char* aura_str_find(char* str, const char* pattern) {
    return strstr(str, pattern);
}

int aura_net_setup(int port) {
#ifdef _WIN32
    WSADATA wsa;
    WSAStartup(MAKEWORD(2, 2), &wsa);
#endif

    int sock = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(port);

    bind(sock, (struct sockaddr*)&addr, sizeof(addr));
    listen(sock, 5);
    printf("Aura Runtime: Listening on port %d\n", port);
    return sock;
}

void aura_close_socket(int sock) {
#ifdef _WIN32
    closesocket(sock);
#else
    close(sock);
#endif
}
